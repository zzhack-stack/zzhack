use crate::utils::{gray_matter::get_post_front_matter, post::get_markdown_path};
use chrono::{DateTime, Utc};
use database::{
    connection::execute,
    rusqlite::{params, Result},
};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
    time::SystemTime,
};

fn format_system_time_to_rfc2822(time: SystemTime) -> String {
    let time: DateTime<Utc> = time.into();

    time.to_rfc2822()
}

pub fn map_posts_to_db() -> Result<()> {
    execute(|conn| {
        let site_config = site_config::get_site_config();

        for read_dir in read_dir(PathBuf::from("..").join(site_config.root.posts_folder_name))
            .expect("Please make sure the posts folder exists")
        {
            let dir_entry = read_dir.unwrap();
            let metadata = dir_entry.metadata().unwrap();
            let create_at = format_system_time_to_rfc2822(metadata.created().unwrap());
            let updated_at = format_system_time_to_rfc2822(metadata.modified().unwrap());
            let path = get_markdown_path(dir_entry.path());
            let content = read_to_string(path.clone()).unwrap();
            let front_matter = get_post_front_matter(&content);

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
                    path.to_string_lossy().to_string(),
                    content,
                    front_matter.title,
                    front_matter.spoiler,
                    create_at,
                    updated_at
                ),
            )?;
        }

        Ok(())
    })
}
