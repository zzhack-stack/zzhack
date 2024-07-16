use database::rusqlite;

use crate::dao::{self};
use models::post::Post;

pub fn get_posts_by_page(page_limit: usize, page: usize) -> rusqlite::Result<Vec<Post>> {
    let page = (page - 1) * page_limit;
    let posts = dao::post::get_posts_by_page(page, page_limit)?
        .into_iter()
        .map(|post| post.unwrap())
        .collect::<Vec<Post>>();

    Ok(posts)
}

pub fn get_posts_count() -> rusqlite::Result<usize> {
    let posts_count = dao::post::get_posts_count()?;

    Ok(posts_count)
}
