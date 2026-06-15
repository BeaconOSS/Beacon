use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;

use crate::error::error;
use crate::session::{self, SessionUser};
pub struct AuthUser(pub SessionUser);

impl<S> FromRequestParts<S> for AuthUser
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let Some(token) = jar.get(session::SESSION_COOKIE).map(|c| c.value().to_string()) else {
            return Err(error(StatusCode::UNAUTHORIZED, "not signed in").into_response());
        };

        let pool = PgPool::from_ref(state);

        match session::lookup(&pool, &token).await {
            Ok(Some(user)) => Ok(AuthUser(user)),
            Ok(None) => Err(error(StatusCode::UNAUTHORIZED, "not signed in").into_response()),
            Err(_) => Err(
                error(StatusCode::INTERNAL_SERVER_ERROR, "could not read session").into_response(),
            ),
        }
    }
}
