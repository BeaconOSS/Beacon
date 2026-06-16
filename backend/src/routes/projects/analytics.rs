use axum::response::{IntoResponse, Response};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use sqlx::Row;

use crate::error::AppError;
use crate::extract::AuthUser;
use crate::routes::owner::require_project_owner;

const RANGE_DAYS: i32 = 30;

#[derive(Serialize)]
struct DayStat {
    day: String,
    views: i32,
    downloads: i32,
}

pub async fn analytics(
    State(pool): State<sqlx::PgPool>,
    AuthUser(user): AuthUser,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let project_id = require_project_owner(&pool, &slug, &user.id).await?;

    let rows = sqlx::query(
        r#"
        select
            to_char(d.day, 'YYYY-MM-DD') as day,
            coalesce(s.views, 0) as views,
            coalesce(s.downloads, 0) as downloads
        from generate_series(
            (now() at time zone 'utc')::date - ($2::int - 1),
            (now() at time zone 'utc')::date,
            interval '1 day'
        ) as d(day)
        left join project_daily_stats s
            on s.project_id = $1::uuid and s.day = d.day::date
        order by d.day
        "#,
    )
    .bind(&project_id)
    .bind(RANGE_DAYS)
    .fetch_all(&pool)
    .await?;

    let mut total_views = 0_i64;
    let mut total_downloads = 0_i64;
    let series: Vec<DayStat> = rows
        .into_iter()
        .map(|row| {
            let views: i32 = row.get("views");
            let downloads: i32 = row.get("downloads");
            total_views += views as i64;
            total_downloads += downloads as i64;
            DayStat {
                day: row.get("day"),
                views,
                downloads,
            }
        })
        .collect();

    let totals = sqlx::query("select download_count from projects where id = $1::uuid")
        .bind(&project_id)
        .fetch_one(&pool)
        .await?;
    let all_time_downloads: i64 = totals.get("download_count");

    let body = json!({
        "range_days": RANGE_DAYS,
        "total_views": total_views,
        "total_downloads": total_downloads,
        "all_time_downloads": all_time_downloads,
        "series": series,
    });

    Ok((StatusCode::OK, Json(body)).into_response())
}
