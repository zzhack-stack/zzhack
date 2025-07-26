// Utility functions for file operations and other common tasks

pub mod app_config;
pub mod config;
pub mod file_utils;
pub mod syntax_highlighter;

pub use app_config::AppConfigService;
pub use config::build_url;
pub use file_utils::{fetch_file_content, fetch_and_render_markdown_with_executor};