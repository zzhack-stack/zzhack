use database::{
    connection::execute,
    rusqlite::{self, params},
};
use shared::post::{Post, PostDetail, RawPost};

pub fn get_post_detail(id: usize) -> rusqlite::Result<PostDetail> {
    execute(|conn| -> rusqlite::Result<PostDetail> {
        let post_detail = conn.query_row(
            "SELECT id, title, content, created_at, updated_at FROM posts
            WHERE id = ?1",
            [id],
            |row| {
                Ok(PostDetail {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    created_at: row.get(3)?,
                    updated_at: row.get(4)?,
                })
            },
        )?;

        Ok(post_detail)
    })
}

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
            "SELECT id, path, title, spoiler, created_at, updated_at FROM posts LIMIT ?1 OFFSET ?2",
        )?;
        let posts_rows = statement
            .query_map(params!(page_limit, page), |row| {
                Ok(Post {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    title: row.get(2)?,
                    spoiler: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            })?
            .collect::<Vec<rusqlite::Result<Post>>>();

        Ok(posts_rows)
    })
}

pub fn delete_posts_by_paths(local_paths: &Vec<String>) -> rusqlite::Result<usize> {
    let local_paths_stringify = &local_paths.join(",");

    execute(|conn| -> rusqlite::Result<usize> {
        conn.execute(
            &format!(
                "DELETE FROM posts
                WHERE path NOT IN ({})",
                local_paths_stringify
            ),
            [],
        )
    })
}

pub fn upsert_post(post: RawPost) -> rusqlite::Result<usize> {
    execute(|conn| -> rusqlite::Result<usize> {
        conn.execute(
            "INSERT INTO posts (path, content, title, spoiler, created_at, updated_at)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(path) DO UPDATE SET
                    content=excluded.content,
                    title=excluded.title,
                    spoiler=excluded.spoiler,
                    created_at=excluded.created_at,
                    updated_at=excluded.updated_at",
            params!(
                post.path.to_string(),
                markdown::parse::parse_markdown(&post.content),
                post.title,
                post.spoiler,
                post.created_at,
                post.updated_at
            ),
        )
    })
}
