use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router, extract::Query, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/categories", get(list))
}

#[derive(Serialize)]
struct Category {
    id: String,
    slug: String,
    name: String,
    project_type: String,
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    project_type: Option<String>,
}

async fn list(
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<CategoryQuery>,
) -> Result<Response, AppError> {
    let rows = sqlx::query(
        r#"
        select id::text as id, slug, name, project_type
        from categories
        where ($1::text is null or project_type = $1)
        order by project_type, ordering
        "#,
    )
    .bind(query.project_type.as_deref())
    .fetch_all(&pool)
    .await?;

    let categories: Vec<Category> = rows
        .into_iter()
        .map(|row| Category {
            id: row.get("id"),
            slug: row.get("slug"),
            name: row.get("name"),
            project_type: row.get("project_type"),
        })
        .collect();
    Ok((StatusCode::OK, Json(json!({ "categories": categories }))).into_response())
}
