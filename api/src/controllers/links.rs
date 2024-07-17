use axum::{http::StatusCode, Json};
use shared::links::LinksConfig;

use crate::{error::ResponseResultExt, services::links_service};

pub async fn get_links() -> Result<Json<LinksConfig>, (StatusCode, String)> {
    let links_config =
        links_service::get_links_config().into_response_result(StatusCode::BAD_REQUEST)?;

    Ok(Json(links_config))
}
