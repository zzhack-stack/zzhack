// Command System Module
// Modular command system with organized command categories

pub mod executor;
pub mod filesystem;
pub mod system;
pub mod types;
pub mod utility;

// Re-export main types and executor
pub use executor::CommandExecutor;
pub use types::{Command, CommandResult, TerminalContext};

// Command implementations are available through their respective modules