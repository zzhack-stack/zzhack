// Filesystem Module
// Virtual filesystem with modular organization

pub mod completion;
pub mod filesystem;
pub mod navigation;
pub mod operations;
pub mod types;

// Re-export main filesystem for backward compatibility
pub use filesystem::FileSystem;