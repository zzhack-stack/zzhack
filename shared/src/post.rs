use serde::{Deserialize, Serialize};

use crate::tag::Tag;

#[derive(Serialize, Deserialize, Clone)]
pub struct PostWithTags<P, T> {
    pub post: P,
    pub tags: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PaginationPostsRes<T> {
    pub page_limit: u64,
    pub page: u64,
    pub total: u64,
    pub has_next: bool,
    pub posts: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub path: String,
    pub spoiler: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostDetail {
    pub id: i32,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub tags: Vec<Tag>,
}

pub trait IntoPost<P>: Sized {
    fn into_post<T: Into<Tag>>(self, tags: Vec<T>) -> P;
}
