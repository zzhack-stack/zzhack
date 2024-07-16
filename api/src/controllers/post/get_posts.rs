use axum::{extract::Query, http::StatusCode, Json};
use models::post::Post;
use serde::{Deserialize, Serialize};

use crate::{dao::post::get_posts_count, services::post_service::get_posts_by_page};

#[derive(Deserialize)]
pub struct Pagination {
    page_limit: usize,
    page: usize,
}

#[derive(Serialize)]
pub struct PostsRes {
    page_limit: usize,
    page: usize,
    total: usize,
    has_next: bool,
    posts: Vec<Post>,
}

pub async fn get_posts(pagination: Query<Pagination>) -> Result<Json<PostsRes>, StatusCode> {
    let posts = get_posts_by_page(pagination.page_limit, pagination.page)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let total = get_posts_count().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let has_next = (pagination.page + 1) * pagination.page_limit < total;

    Ok(Json(PostsRes {
        page_limit: pagination.page_limit,
        page: pagination.page,
        posts,
        has_next,
        total,
    }))
}
