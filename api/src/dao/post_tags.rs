use crate::database::{
    connection::DBResult,
    models::post_tags::{ActiveModel, Column, Entity, Model},
};
use sea_orm::{sea_query::OnConflict, ConnectionTrait, EntityTrait, InsertResult, Set};

pub async fn upsert_post_tags<T: ConnectionTrait>(
    db: &T,
    tag_ids: Vec<i64>,
    post_id: i64,
) -> DBResult<InsertResult<ActiveModel>> {
    Entity::insert_many(
        tag_ids
            .into_iter()
            .map(|tag_id| ActiveModel {
                tag_id: Set(tag_id),
                post_id: Set(post_id),
                ..Default::default()
            })
            .collect::<Vec<ActiveModel>>(),
    )
    .on_conflict(
        OnConflict::columns([Column::TagId, Column::PostId])
            .do_nothing()
            .to_owned(),
    )
    .exec(db)
    .await
}
