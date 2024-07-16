use models::post::Post;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PaginationPostsRes {
    pub page_limit: usize,
    pub page: usize,
    pub total: usize,
    pub has_next: bool,
    pub posts: Vec<Post>,
}
