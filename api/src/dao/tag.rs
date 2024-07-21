use std::future::Future;

use futures::future::join_all;
use sea_orm::{
    sea_query::OnConflict, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult,
    Set, TransactionError, TransactionTrait,
};
use shared::tag::Tag;

use crate::database::{
    connection::DBResult,
    models::{
        prelude::Posts,
        tags::{ActiveModel, Column, Entity, Model},
    },
};

use super::post_tags::upsert_post_tags;

async fn upsert_tags<T: ConnectionTrait>(db: &T, tags: Vec<String>) -> Vec<i32> {
    let tags_active_model_futures = tags.into_iter().map(|tag| async {
        Entity::insert(ActiveModel {
            text: Set(tag),
            ..Default::default()
        })
        .on_conflict(OnConflict::column(Column::Text).do_nothing().to_owned())
        .exec(db)
        .await
    });

    join_all(tags_active_model_futures)
        .await
        .into_iter()
        .map(|insert_result| insert_result.unwrap().last_insert_id)
        .collect::<Vec<i32>>()
}

pub async fn upsert_tags_with_post_id(
    db: &DatabaseConnection,
    tags: Vec<String>,
    post_id: i32,
) -> DBResult<(), TransactionError<DbErr>> {
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            let tag_ids = upsert_tags(txn, tags).await;
            upsert_post_tags(txn, tag_ids, post_id).await;

            Ok(())
        })
    })
    .await
}

pub async fn get_all_tags(db: &DatabaseConnection) -> DBResult<Vec<Model>> {
    Entity::find().all(db).await
}

pub async fn get_tags_by_post_id(db: &DatabaseConnection, post_id: i32) -> DBResult<Vec<Model>> {
    let results = Posts::find_by_id(post_id)
        .find_with_related(Entity)
        .all(db)
        .await?;

    if results.len() == 0 {
        Ok(vec![])
    } else {
        let (_, tags) = results[0].clone();

        Ok(tags)
    }
}

impl Into<Tag> for Model {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            text: self.text,
        }
    }
}
