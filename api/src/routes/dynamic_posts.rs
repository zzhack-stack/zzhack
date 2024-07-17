use axum::{routing::get, Router};

use crate::controllers::dynamic::get_dynamic_post_content;

pub fn get_dynamic_posts_routes() -> Router {
    Router::new().route("/post/*path", get(get_dynamic_post_content))
}
