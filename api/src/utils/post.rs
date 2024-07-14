use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

use super::gray_matter::get_post_front_matter;

const DEFAULT_POST_NAME: &'static str = "index.md";

// Basically the type of posts are combine with directory or single Markdown file
// if the path is directory, return self/index.md.
// if the path is markdown file return itself
// if the path is markdown file without extension, return itself.md
pub fn get_markdown_path(post_path: PathBuf) -> PathBuf {
    if post_path.is_dir() {
        return post_path.join(DEFAULT_POST_NAME);
    }

    let mut os_string = post_path.clone().into_os_string();

    if os_string.to_str().unwrap().ends_with(".md") {
        return post_path;
    }

    os_string.push(".md");
    os_string.into()
}

const AVERAGE_WORD_PER_MINUTE: usize = 400;
const MIN_READ_MINUTES: usize = 1;

// Calculate the word per minute
pub fn calc_read_minutes(content: &str) -> usize {
    let read_minutes = content.len() / AVERAGE_WORD_PER_MINUTE;

    if read_minutes < MIN_READ_MINUTES {
        MIN_READ_MINUTES
    } else {
        read_minutes
    }
}

struct PostBriefInfo {
    name: String,
    create_at: String,
    spoiler: String,
    read_minutes: usize,
}

fn get_posts_list(current_page: usize) -> Vec<PostBriefInfo> {
    read_dir("../posts")
        .unwrap()
        .map(|post| {
            let post_entry = post.unwrap();
            let content = read_to_string(get_markdown_path(post_entry.path())).unwrap();
            let gray_matter = get_post_front_matter(&content);
            let read_minutes = calc_read_minutes(&content);

            PostBriefInfo {
                name: gray_matter.title,
                spoiler: gray_matter.spoiler,
                create_at: "Tue Jul 09 2024 20:48:31 GMT+0800 (China Standard Time)".to_string(),
                read_minutes,
            }
        })
        .collect::<Vec<PostBriefInfo>>()
}
