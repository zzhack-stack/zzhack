use crate::{
    dao::post::{delete_posts_by_paths, upsert_post},
    utils::{gray_matter::get_post_front_matter, post::get_markdown_path},
};
use chrono::{DateTime, Utc};
use sea_orm::{DatabaseConnection, Set};
use std::{
    fs::{read_dir, read_to_string, DirEntry},
    path::PathBuf,
    time::SystemTime,
};

use super::models::posts::ActiveModel;

fn format_system_time_to_rfc2822(time: SystemTime) -> String {
    let time: DateTime<Utc> = time.into();

    time.to_rfc2822()
}

async fn upsert_posts(db: &DatabaseConnection, dir_entries: &Vec<DirEntry>) -> anyhow::Result<()> {
    for dir_entry in dir_entries {
        let metadata = dir_entry.metadata()?;
        let created_at = format_system_time_to_rfc2822(metadata.created().unwrap());
        let updated_at = format_system_time_to_rfc2822(metadata.modified().unwrap());
        let dir_path = dir_entry.path();
        let path = get_markdown_path(dir_path.clone());
        let content = read_to_string(path.clone())?;
        let front_matter = get_post_front_matter(&content);

        upsert_post(
            db,
            ActiveModel {
                path: Set(dir_path.to_string_lossy().to_string()),
                content: Set(markdown::parse::parse_markdown(&content)),
                title: Set(front_matter.title),
                spoiler: Set(Some(front_matter.spoiler)),
                created_at: Set(created_at),
                updated_at: Set(updated_at),
                ..Default::default()
            },
        )
        .await?;
    }

    Ok(())
}

// Delete posts that is not in local_paths, which means these posts has been
// deleted on disk, so we need to delete corresponding post from database
async fn delete_posts(
    conn: &DatabaseConnection,
    dir_entries: &Vec<DirEntry>,
) -> anyhow::Result<()> {
    let posts_paths = dir_entries
        .into_iter()
        .map(|dir_entry| dir_entry.path().to_string_lossy().to_string())
        .collect::<Vec<String>>();

    delete_posts_by_paths(conn, &posts_paths).await?;

    Ok(())
}

// Regularly users just need to modified the posts folder to
// create post, the function will map these posts into database
// thus, users can access these posts through API
pub async fn map_posts_to_db(conn: &DatabaseConnection) -> anyhow::Result<()> {
    let site_config = site_config::get_site_config();
    let dir_entries = read_dir(PathBuf::from("..").join(site_config.root.posts_folder_name))
        .expect("Please make sure the posts folder exists")
        .map(|dir_entry| dir_entry.unwrap())
        .collect::<Vec<DirEntry>>();

    upsert_posts(conn, &dir_entries).await?;
    delete_posts(conn, &dir_entries).await?;

    Ok(())
}
