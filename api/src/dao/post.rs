use database::{
    connection::execute,
    rusqlite::{self, params},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub path: String,
    pub content: String,
    pub spoiler: String,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn get_posts_by_page(page: usize, page_limit: usize) -> rusqlite::Result<Vec<Post>> {
    execute(move |conn| {
        let mut statement = conn.prepare("SELECT * FROM posts LIMIT ?1 OFFSET ?2")?;
        let posts_rows = statement
            .query_map(params!(page_limit, page), |row| {
                Ok(Post {
                    path: row.get(1)?,
                    content: row.get(2)?,
                    title: row.get(3)?,
                    spoiler: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            })?
            .map(|post| post.unwrap())
            .collect::<Vec<Post>>();

        Ok(posts_rows)
    })
}
