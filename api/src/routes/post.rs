use crate::{
    controllers::post::{get_post_detail::get_post_detail, get_posts::get_posts},
    AppState,
};
use axum::{routing::get, Router};

pub fn get_posts_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_posts))
        .route("/:id", get(get_post_detail))
}
