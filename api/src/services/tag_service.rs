use sea_orm::DatabaseConnection;
use shared::tag::Tag;

use crate::{
    dao::{self, tag},
    utils::vector_convert::convert_vecs,
};

pub async fn get_all_tags(
    db: &DatabaseConnection,
    post_id: Option<i32>,
) -> anyhow::Result<Vec<Tag>> {
    let tags = match post_id {
        Some(post_id) => dao::tag::get_tags_by_post_id(db, post_id).await,
        None => tag::get_all_tags(db).await,
    }?;

    Ok(convert_vecs(tags))
}
