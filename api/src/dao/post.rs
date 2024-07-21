use crate::utils::vector_convert::convert_vecs;
use sea_orm::{
    sea_query::OnConflict, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait,
    InsertResult, PaginatorTrait, QueryFilter, QuerySelect,
};
use shared::post::{IntoPost, Post};

use crate::database::{
    connection::DBResult,
    models::posts::{ActiveModel, Column, Entity, Model},
};

pub async fn get_post_by_id(db: &DatabaseConnection, id: i32) -> DBResult<Model> {
    let post_detail = Entity::find_by_id(id).one(db).await?.unwrap();

    Ok(post_detail)
}

pub async fn get_post_by_path(db: &DatabaseConnection, path: &str) -> DBResult<Option<Model>> {
    Entity::find().filter(Column::Path.eq(path)).one(db).await
}

pub async fn get_posts_count(db: &DatabaseConnection) -> DBResult<u64> {
    Entity::find().count(db).await
}

pub async fn get_posts_by_page(
    db: &DatabaseConnection,
    page: u64,
    page_limit: u64,
) -> DBResult<Vec<Model>> {
    Entity::find()
        .columns([
            Column::Id,
            Column::Path,
            Column::Title,
            Column::Spoiler,
            Column::CreatedAt,
            Column::UpdatedAt,
        ])
        .to_owned()
        .paginate(db, page_limit)
        .fetch_page(page)
        .await
}

pub async fn delete_posts_by_paths(
    db: &DatabaseConnection,
    local_paths: &Vec<String>,
) -> DBResult<DeleteResult> {
    Entity::delete_many()
        .filter(Column::Path.is_not_in(local_paths))
        .exec(db)
        .await
}

pub async fn upsert_post(
    db: &DatabaseConnection,
    post: ActiveModel,
) -> DBResult<InsertResult<ActiveModel>> {
    Entity::insert(post)
        .on_conflict(
            OnConflict::column(Column::Path)
                .update_columns([
                    Column::Content,
                    Column::Title,
                    Column::Spoiler,
                    Column::UpdatedAt,
                ])
                .to_owned(),
        )
        .exec(db)
        .await
}

impl IntoPost<Post> for Model {
    fn into_post<T: Into<shared::tag::Tag>>(self, tags: Vec<T>) -> Post {
        Post {
            id: self.id,
            path: self.path,
            spoiler: self.spoiler.unwrap_or_default(),
            title: self.title,
            created_at: self.created_at,
            updated_at: self.updated_at,
            tags: convert_vecs(tags),
        }
    }
}
