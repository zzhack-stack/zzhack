use axum::{routing::get, Router};

use crate::{controllers::dynamic::get_dynamic_post_content, AppState};

pub fn get_dynamic_posts_routes() -> Router<AppState> {
    Router::new().route("/post/*path", get(get_dynamic_post_content))
}
