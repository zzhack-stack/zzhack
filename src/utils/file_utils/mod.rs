// File Operations Module
// Modular file handling with separate concerns

pub mod fetcher;
pub mod markdown_processor;

// Re-export main functions for backward compatibility
pub use fetcher::fetch_file_content;
pub use markdown_processor::fetch_and_render_markdown_with_executor;