use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize)]
#[cfg_attr(feature = "with-json", derive(Serialize, Deserialize))]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub path: String,
    pub content: String,
    pub title: String,
    #[sea_orm(nullable)]
    pub spoiler: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::post_tag::Relation::Tags.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::post_tag::Relation::Posts.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
