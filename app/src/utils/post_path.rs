use std::path::PathBuf;

const DEFAULT_POST_NAME: &'static str = "index.md";

// Basically the type of posts are combine with directory or single Markdown file
// if the path is directory, return self/index.md.
// if the path is markdown file return itself
// if the path is markdown file without extension, return itself.md
pub fn get_markdown_path(post_path: PathBuf) -> PathBuf {
    if post_path.is_dir() {
        return post_path.join(DEFAULT_POST_NAME);
    }

    if post_path.ends_with(".md") {
        return post_path;
    }

    let mut os_string = post_path.into_os_string();
    os_string.push(".md");

    os_string.into()
}
