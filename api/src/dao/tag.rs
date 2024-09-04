use futures::future::join_all;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    sea_query::OnConflict, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait, Set,
    TransactionError, TransactionTrait,
};
use shared::tag::Tag;

use crate::utils::gray_matter;
use crate::{
    database::{
        connection::DBResult,
        models::{
            prelude::Posts,
            tags::{ActiveModel, Column, Entity, Model},
        },
    },
    utils::helpers::parse_load_many_result,
};

use super::post_tags::insert_post_tags;

impl Into<ActiveModel> for &gray_matter::Tag {
    fn into(self) -> ActiveModel {
        match self {
            gray_matter::Tag::Text(text) => ActiveModel {
                text: Set(text.to_string()),
                color: NotSet,
                ..Default::default()
            },
            gray_matter::Tag::CustomColor { text, color } => ActiveModel {
                text: Set(text.to_string()),
                color: Set(Some(color.to_string())),
                ..Default::default()
            },
        }
    }
}

async fn upsert_tags<T: ConnectionTrait>(db: &T, tags: &Vec<gray_matter::Tag>) -> Vec<i32> {
    let tags_active_model_futures = tags.into_iter().map(|tag| async {
        let active_model: ActiveModel = tag.into();

        Entity::insert(active_model)
            .on_conflict(
                OnConflict::column(Column::Text)
                    .update_column(Column::Color)
                    .to_owned(),
            )
            .exec(db)
            .await
    });

    join_all(tags_active_model_futures)
        .await
        .into_iter()
        // Performing an upsert statement without inserting or updating any of the row will result in a DbErr::RecordNotInserted error.
        .filter(|insert_result| !matches!(insert_result, Err(DbErr::RecordNotInserted)))
        .map(|insert_result| insert_result.unwrap().last_insert_id)
        .collect::<Vec<i32>>()
}

pub async fn upsert_tags_with_post_id(
    db: &DatabaseConnection,
    tags: Vec<gray_matter::Tag>,
    post_id: i32,
) -> DBResult<(), TransactionError<DbErr>> {
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            upsert_tags(txn, &tags).await;
            insert_post_tags(txn, &tags, post_id).await;

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

    Ok(parse_load_many_result(results))
}

impl Into<Tag> for Model {
    fn into(self) -> Tag {
        Tag {
            id: self.id,
            text: self.text,
            color: self.color,
        }
    }
}
