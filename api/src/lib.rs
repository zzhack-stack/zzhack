pub mod controllers;
pub mod dao;
mod error;
mod routes;
pub mod services;
pub mod utils;

use axum::Router;
use routes::{
    dynamic_posts::get_dynamic_posts_routes, links::get_links_routes, post::get_posts_routes,
};

pub fn get_api_routes() -> Router {
    Router::new()
        .nest("/posts", get_posts_routes())
        .nest("/links", get_links_routes())
        .nest("/dynamic", get_dynamic_posts_routes())
}
