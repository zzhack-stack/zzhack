use axum::{extract::Query, http::StatusCode, Json};
use response::post::PaginationPostsRes;
use serde::Deserialize;

use crate::{
    error::ResponseResultExt,
    services::post_service::{get_posts_by_page, get_posts_count},
};

#[derive(Deserialize)]
pub struct Pagination {
    page_limit: usize,
    page: usize,
}

pub async fn get_posts(
    pagination: Query<Pagination>,
) -> Result<Json<PaginationPostsRes>, (StatusCode, String)> {
    let posts = get_posts_by_page(pagination.page_limit, pagination.page)
        .into_response_result(StatusCode::BAD_REQUEST)?;
    let total = get_posts_count().into_response_result(StatusCode::INTERNAL_SERVER_ERROR)?;
    let has_next = (pagination.page + 1) * pagination.page_limit <= total;

    Ok(Json(PaginationPostsRes {
        page_limit: pagination.page_limit,
        page: pagination.page,
        posts,
        has_next,
        total,
    }))
}
