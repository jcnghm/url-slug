use crate::database;
use crate::models::{CreateSlugRequest, SlugResponse, SlugStatsResponse};
use crate::slug::{generate_slug, is_valid_url};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{Json as JsonResponse, Redirect},
};
use sqlx::SqlitePool;

pub async fn create_slug(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateSlugRequest>,
) -> Result<JsonResponse<SlugResponse>, StatusCode> {
    if !is_valid_url(&payload.url) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let slug = generate_slug();

    if let Err(_) = database::insert_slug(&pool, &slug, &payload.url).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(JsonResponse(SlugResponse {
        slug: slug.clone(),
        short_url: format!("http://localhost:3000/{}", slug),
    }))
}

pub async fn redirect_slug(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<Redirect, StatusCode> {
    match database::get_url_and_increment(&pool, &slug).await {
        Ok(Some(url)) => Ok(Redirect::permanent(&url)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_slug_stats(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<JsonResponse<SlugStatsResponse>, StatusCode> {
    match database::get_slug_stats(&pool, &slug).await {
        Ok(Some((url, count))) => Ok(JsonResponse(SlugStatsResponse {
            original_url: url,
            slug,
            click_count: count,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
