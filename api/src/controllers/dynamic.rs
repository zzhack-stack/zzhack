use axum::{extract::Path, http::StatusCode, Json};

use crate::{
    error::ResponseResultExt, services::dynamic_posts_service::get_dynamic_post_rendered_content,
};

pub async fn get_dynamic_post_content(
    Path(path): Path<String>,
) -> Result<String, (StatusCode, String)> {
    let rendered_content =
        get_dynamic_post_rendered_content(&path).into_response_result(StatusCode::BAD_REQUEST)?;

    Ok(rendered_content)
}
