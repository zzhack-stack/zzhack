use crate::{
    database::models::{
        post_tags::{ActiveModel, Column, Entity},
        tags::{Column as TagsColumn, Entity as Tags},
    },
    utils::helpers::{filter_record_not_insert_error, parse_load_many_result},
};
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, Set,
};

pub async fn upsert_post_tags<T: ConnectionTrait>(db: &T, tags: &Vec<String>, post_id: i32) {
    let tag_ids = Tags::find()
        .filter(TagsColumn::Text.is_in(tags))
        .all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|tag| tag.id)
        .collect::<Vec<i32>>();

    filter_record_not_insert_error(
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
        .await,
    );
}

// When users delete some tag from a post, we should delete the corresponding row in the post_tags table.
// And if the deleted tag is not used by any other post, we should also delete the tag from the tags table.
pub async fn delete_tags_by_post_id<T: ConnectionTrait>(db: &T, tags: &Vec<String>, post_id: i32) {
    let need_delete_tags = Entity::find()
        .filter(Column::PostId.eq(post_id))
        .find_with_related(Tags)
        .filter(TagsColumn::Text.is_not_in(tags))
        .all(db)
        .await
        .unwrap();

    let need_delete_tag_ids = parse_load_many_result(need_delete_tags)
        .into_iter()
        .map(|tag| tag.id)
        .collect::<Vec<i32>>();

    // Delete tags in tags table, which has been deleted from disk
    Entity::delete_many()
        .filter(
            Condition::all()
                .add(Column::PostId.eq(post_id))
                .add(Column::TagId.is_in(need_delete_tag_ids)),
        )
        .exec(db)
        .await
        .unwrap();
}
