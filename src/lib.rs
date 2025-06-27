// Terminal Emulator Library
// This is the main library crate that exports the core application components
// for a browser-based terminal emulator built with Rust and Yew

/// Application module containing the main App component
pub mod app;

/// UI components module containing reusable terminal UI elements
pub mod components;

/// Command execution module containing built-in terminal commands
pub mod commands;

/// Local filesystem module for metadata-based file operations
pub mod filesystem;

// Re-export the main App component for easy access
pub use app::App;

// When building as a binary, also re-export for main.rs
#[cfg(not(feature = "lib-only"))]
pub use app::App as AppComponent;