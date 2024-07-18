use crate::{
    database::models::posts::Model, error::ResponseResultExt, services::post_service, AppState,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_post_detail(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Model>, (StatusCode, String)> {
    let detail = post_service::get_post_detail(&state.conn, id)
        .await
        .into_response_result(StatusCode::BAD_REQUEST)?;

    Ok(Json(detail))
}
