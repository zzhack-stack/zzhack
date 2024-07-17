use crate::controllers::links::get_links;
use axum::{routing::get, Router};

pub fn get_links_routes() -> Router {
    Router::new().route("/", get(get_links))
}
