use std::future::Future;

use futures::future::join_all;
use sea_orm::{
    sea_query::OnConflict, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, InsertResult,
    Set, TransactionError, TransactionTrait,
};

use crate::database::{
    connection::DBResult,
    models::tags::{ActiveModel, Column, Entity, Model},
};

use super::post_tags::upsert_post_tags;

async fn upsert_tags<T: ConnectionTrait>(db: &T, tags: Vec<String>) -> Vec<i64> {
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
        .map(|insert_result| insert_result.unwrap().last_insert_id as i64)
        .collect::<Vec<i64>>()
}

pub async fn upsert_tags_with_post_id(
    db: &DatabaseConnection,
    tags: Vec<String>,
    post_id: i64,
) -> DBResult<(), TransactionError<DbErr>> {
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            let tag_ids = upsert_tags(txn, tags).await;
            upsert_post_tags(txn, tag_ids, post_id).await?;

            Ok(())
        })
    })
    .await
}
