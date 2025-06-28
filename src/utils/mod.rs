// Utility functions for file operations and other common tasks

pub mod config;
pub mod file_utils;

pub use config::{get_base_url, build_url, build_data_url};
pub use file_utils::{fetch_file_content, fetch_and_render_markdown};