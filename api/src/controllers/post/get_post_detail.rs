use axum::{extract::Path, http::StatusCode, Json};
use shared::post::PostDetail;

use crate::{error::ResponseResultExt, services::post_service};

pub async fn get_post_detail(
    Path(id): Path<usize>,
) -> Result<Json<PostDetail>, (StatusCode, String)> {
    let detail = post_service::get_post_detail(id).into_response_result(StatusCode::BAD_REQUEST)?;

    Ok(Json(detail))
}
