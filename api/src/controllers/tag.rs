use crate::{error::ResponseResultExt, services::tag_service::get_all_tags, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use shared::tag::Tag;

#[derive(Deserialize)]
pub struct QueryParams {
    post_id: Option<i32>,
}

pub async fn get_tags(
    state: State<AppState>,
    query_params: Query<QueryParams>,
) -> Result<Json<Vec<Tag>>, (StatusCode, String)> {
    let tags = get_all_tags(&state.conn, query_params.post_id)
        .await
        .into_response_result(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tags))
}
