use crate::dao::{self};
use crate::database::models::posts::Model;
use anyhow::{bail, Result};
use sea_orm::DatabaseConnection;

pub async fn get_posts_by_page(
    conn: &DatabaseConnection,
    page_limit: u64,
    page: u64,
) -> Result<Vec<Model>> {
    // if page <= 0 {
    //     bail!("The page should greater than 0");
    // }

    let posts = dao::post::get_posts_by_page(conn, page, page_limit).await?;

    Ok(posts)
}

pub async fn get_posts_count(conn: &DatabaseConnection) -> Result<u64> {
    let posts_count = dao::post::get_posts_count(conn).await?;

    Ok(posts_count)
}

pub async fn get_post_detail(conn: &DatabaseConnection, id: i32) -> Result<Model> {
    let post_detail = dao::post::get_post_by_id(conn, id).await?;

    Ok(post_detail)
}
