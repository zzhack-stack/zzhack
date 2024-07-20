use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use shared::post::{PaginationPostsRes, PostWithTags};

use crate::{
    database::models::{posts::Model, tags},
    error::ResponseResultExt,
    services::post_service::{get_posts_by_page, get_posts_count},
    AppState,
};

#[derive(Deserialize)]
pub struct Pagination {
    page_limit: u64,
    page: u64,
}

pub async fn get_posts(
    state: State<AppState>,
    pagination: Query<Pagination>,
) -> Result<Json<PaginationPostsRes<PostWithTags<Model, tags::Model>>>, (StatusCode, String)> {
    let posts_with_tags = get_posts_by_page(&state.conn, pagination.page_limit, pagination.page)
        .await
        .into_response_result(StatusCode::BAD_REQUEST)?;
    let total = get_posts_count(&state.conn)
        .await
        .into_response_result(StatusCode::INTERNAL_SERVER_ERROR)?;
    let has_next = (pagination.page + 1) * pagination.page_limit < total;

    Ok(Json(PaginationPostsRes {
        page_limit: pagination.page_limit,
        page: pagination.page,
        posts_with_tags,
        has_next,
        total,
    }))
}
