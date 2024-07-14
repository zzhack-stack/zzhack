use database::rusqlite;

use crate::dao::{self, post::Post};

pub mod post_service;

pub fn get_posts_by_page(page_limit: usize, page: usize) -> rusqlite::Result<Vec<Post>> {
    let posts = dao::post::get_posts_by_page(page, page_limit)?
        .into_iter()
        .map(|post| post.unwrap())
        .collect::<Vec<Post>>();

    Ok(posts)
}
