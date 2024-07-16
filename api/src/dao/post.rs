use database::{
    connection::execute,
    rusqlite::{self, params},
};
use models::post::Post;

pub fn get_posts_count() -> rusqlite::Result<usize> {
    execute(|conn| -> rusqlite::Result<usize> {
        let row = conn.query_row("SELECT COUNT(*) FROM posts", [], |row| {
            Ok(row.get_unwrap::<_, usize>(0))
        })?;

        println!("{}", row);

        Ok(1)
    })
}

pub fn get_posts_by_page(
    page: usize,
    page_limit: usize,
) -> rusqlite::Result<Vec<rusqlite::Result<Post>>> {
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
            .collect::<Vec<rusqlite::Result<Post>>>();

        Ok(posts_rows)
    })
}
