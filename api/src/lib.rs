pub mod controllers;
pub mod dao;
pub mod database;
mod error;
mod routes;
pub mod services;
pub mod utils;

use axum::Router;
use routes::{
    dynamic_posts::get_dynamic_posts_routes, links::get_links_routes, post::get_posts_routes,
};
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub fn get_api_routes() -> Router<AppState> {
    Router::new()
        .nest("/posts", get_posts_routes())
        .nest("/links", get_links_routes())
        .nest("/dynamic", get_dynamic_posts_routes())
}
