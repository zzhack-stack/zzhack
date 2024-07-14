use axum::{extract::Query, http::StatusCode, Json};
use serde::Deserialize;

use crate::{dao::post::Post, services::get_posts_by_page};

#[derive(Deserialize)]
pub struct Pagination {
    page_limit: usize,
    page: usize,
}

pub async fn get_posts(pagination: Query<Pagination>) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = get_posts_by_page(pagination.page_limit, pagination.page)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}
