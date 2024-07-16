use crate::controllers::post::get_posts::get_posts;
use axum::{routing::get, Router};

pub fn get_posts_routes() -> Router {
    Router::new().route("/", get(get_posts))
}
