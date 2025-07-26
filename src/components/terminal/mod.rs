// Terminal Module
// All terminal-related components organized following React component philosophy

pub mod container;
pub mod content;
pub mod handlers;
pub mod header;
pub mod history;
pub mod hooks;
pub mod input;

// Re-export main components for easier access
pub use container::Terminal;