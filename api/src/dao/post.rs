use database::{
    connection::execute,
    rusqlite::{self, params},
};
use shared::post::Post;

pub fn get_posts_count() -> rusqlite::Result<usize> {
    execute(|conn| -> rusqlite::Result<usize> {
        let count = conn.query_row("SELECT COUNT(*) FROM posts", [], |row| {
            Ok(row.get_unwrap::<_, usize>(0))
        })?;

        Ok(count)
    })
}

pub fn get_posts_by_page(
    page: usize,
    page_limit: usize,
) -> rusqlite::Result<Vec<rusqlite::Result<Post>>> {
    execute(move |conn| {
        let mut statement = conn.prepare(
            "SELECT path, title, spoiler, created_at, updated_at FROM posts LIMIT ?1 OFFSET ?2",
        )?;
        let posts_rows = statement
            .query_map(params!(page_limit, page), |row| {
                Ok(Post {
                    path: row.get(0)?,
                    title: row.get(1)?,
                    spoiler: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            })?
            .collect::<Vec<rusqlite::Result<Post>>>();

        Ok(posts_rows)
    })
}
