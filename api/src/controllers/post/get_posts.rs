use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use shared::post::{PaginationPostsRes, Post};

use crate::{
    error::ResponseResultExt,
    services::post_service::{get_pagination_posts, get_posts_by_tag_id, get_posts_count},
    AppState,
};

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    page_limit: u64,
    page: u64,
    tag_id: Option<i32>,
}

pub async fn get_posts(
    state: State<AppState>,
    query_params: Query<QueryParams>,
) -> Result<Json<PaginationPostsRes<Post>>, (StatusCode, String)> {
    let total = get_posts_count(&state.conn)
        .await
        .into_response_result(StatusCode::INTERNAL_SERVER_ERROR)?;
    let has_next = (query_params.page + 1) * query_params.page_limit < total;

    let posts = match query_params.tag_id {
        None => get_pagination_posts(&state.conn, query_params.page_limit, query_params.page)
            .await
            .into_response_result(StatusCode::BAD_REQUEST),
        Some(tag_id) => get_posts_by_tag_id(
            &state.conn,
            tag_id,
            query_params.page_limit,
            query_params.page,
        )
        .await
        .into_response_result(StatusCode::BAD_REQUEST),
    }?;

    Ok(Json(PaginationPostsRes {
        total,
        has_next,
        page_limit: query_params.page_limit,
        page: query_params.page_limit,
        posts,
    }))
}
