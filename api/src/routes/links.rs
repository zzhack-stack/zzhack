use crate::{controllers::links::get_links, AppState};
use axum::{routing::get, Router};

pub fn get_links_routes() -> Router<AppState> {
    Router::new().route("/", get(get_links))
}
