use crate::dao::tag::get_tags_by_post_id;
use crate::dao::{self};
use crate::database::models::posts::Model;
use anyhow::Result;
use futures::future::join_all;
use sea_orm::DatabaseConnection;
use shared::post::{IntoPost, Post};

pub async fn get_posts_by_page(
    conn: &DatabaseConnection,
    page_limit: u64,
    page: u64,
) -> Result<Vec<Post>> {
    let posts = dao::post::get_posts_by_page(conn, page, page_limit)
        .await?
        .into_iter()
        .map(|post| async {
            let tags = get_tags_by_post_id(conn, post.id).await.unwrap();

            post.into_post(tags)
        });
    let posts_with_tags = join_all(posts).await;

    Ok(posts_with_tags)
}

pub async fn get_posts_count(conn: &DatabaseConnection) -> Result<u64> {
    let posts_count = dao::post::get_posts_count(conn).await?;

    Ok(posts_count)
}

pub async fn get_post_detail(conn: &DatabaseConnection, id: i32) -> Result<Model> {
    let post_detail = dao::post::get_post_by_id(conn, id).await?;

    Ok(post_detail)
}
