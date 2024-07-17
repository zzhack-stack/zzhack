use anyhow::Result;
use markdown::parse::parse_markdown;
use std::{fs::read_to_string, path::PathBuf};

pub fn get_dynamic_post_rendered_content(path: &str) -> Result<String> {
    let config = site_config::get_site_config();
    let about_me_content = read_to_string(
        PathBuf::from("..")
            .join(config.root.dynamic_pages_folder_name)
            .join(path)
            .with_extension("md"),
    )?;

    Ok(parse_markdown(&about_me_content))
}
