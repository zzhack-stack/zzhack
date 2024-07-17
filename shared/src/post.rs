use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PaginationPostsRes {
    pub page_limit: usize,
    pub page: usize,
    pub total: usize,
    pub has_next: bool,
    pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
