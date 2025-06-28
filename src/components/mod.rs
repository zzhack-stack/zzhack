// Terminal Components Module
// This module contains all terminal-related UI components and utilities

pub mod ansi;
pub mod history;
pub mod syntax;
pub mod terminal;

// Re-export main components for easier access  
pub use terminal::Terminal;