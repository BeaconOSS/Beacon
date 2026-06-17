use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;

use crate::error::AppError;
use crate::session::{self, SessionUser};
pub struct AuthUser(pub SessionUser);

impl<S> FromRequestParts<S> for AuthUser
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_headers(&parts.headers);

        let Some(token) = jar
            .get(session::SESSION_COOKIE)
            .map(|c| c.value().to_string())
        else {
            return Err(AppError::unauthorized("not signed in"));
        };

        let pool = PgPool::from_ref(state);

        match session::lookup(&pool, &token).await {
            Ok(Some(user)) => Ok(AuthUser(user)),
            Ok(None) => Err(AppError::unauthorized("not signed in")),
            Err(_) => Err(AppError::internal("could not read session")),
        }
    }
}

pub struct ModeratorUser(pub SessionUser);

impl<S> FromRequestParts<S> for ModeratorUser
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let AuthUser(user) = AuthUser::from_request_parts(parts, state).await?;
        if user.role == crate::constants::ROLE_MODERATOR
            || user.role == crate::constants::ROLE_ADMIN
        {
            Ok(ModeratorUser(user))
        } else {
            Err(AppError::forbidden("moderator access required"))
        }
    }
}
