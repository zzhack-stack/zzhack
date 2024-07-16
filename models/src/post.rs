use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub path: String,
    pub content: String,
    pub spoiler: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}
