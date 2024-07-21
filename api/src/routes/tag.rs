use crate::{controllers::tag::get_tags, AppState};
use axum::{routing::get, Router};

pub fn get_tags_routes() -> Router<AppState> {
    Router::new().route("/", get(get_tags))
}
