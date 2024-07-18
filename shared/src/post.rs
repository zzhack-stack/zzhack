use serde::{Deserialize, Serialize};

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
    pub id: usize,
    pub path: String,
    pub spoiler: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawPost {
    pub path: String,
    pub content: String,
    pub spoiler: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostDetail {
    pub id: usize,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
}
