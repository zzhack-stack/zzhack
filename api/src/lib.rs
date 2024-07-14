mod controllers;
mod dao;
mod routes;
mod services;
pub mod utils;

use axum::Router;
use routes::post::get_posts_routes;

pub fn get_api_routes() -> Router {
    Router::new().nest("/posts", get_posts_routes())
}
