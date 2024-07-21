use crate::dao::tag::get_tags_by_post_id;
use crate::dao::{self};
use crate::database::models::posts::Model;
use crate::utils::helpers::parse_load_many_result;
use anyhow::{bail, Result};
use futures::future::join_all;
use sea_orm::DatabaseConnection;
use shared::post::{IntoPost, Post, PostDetail};

pub async fn get_pagination_posts(
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

pub async fn get_posts_by_tag_id(
    db: &DatabaseConnection,
    tag_id: i32,
    page_limit: u64,
    page: u64,
) -> Result<Vec<Post>> {
    let result = dao::post::get_posts_by_tag_id(db, tag_id, page_limit, page * page_limit).await?;

    if result.len() == 0 {
        Ok(vec![])
    } else {
        let (tag, posts) = result[0].clone();

        let posts = posts
            .into_iter()
            .map(move |post_model| post_model.into_post(vec![tag.clone()]))
            .collect::<Vec<Post>>();

        Ok(posts)
    }
}

pub async fn get_posts_count(conn: &DatabaseConnection) -> Result<u64> {
    let posts_count = dao::post::get_posts_count(conn).await?;

    Ok(posts_count)
}

pub async fn get_post_detail(conn: &DatabaseConnection, id: i32) -> Result<PostDetail> {
    let result = dao::post::get_post_by_id(conn, id).await?;

    if result.len() == 0 {
        bail!("Cannot find post with id {}", id)
    }

    let (post_model, tags) = result[0].clone();

    Ok(post_model.into_post(tags))
}
