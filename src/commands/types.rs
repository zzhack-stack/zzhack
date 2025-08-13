// Command System Types
// Core types and traits for the command system

use crate::utils::AppConfigService;
use std::future::Future;
use std::pin::Pin;

/// Terminal context providing utility functions for commands
pub struct TerminalContext<'a> {
    pub clear_screen: std::rc::Rc<dyn Fn()>,
    pub command_executor: &'a crate::commands::CommandExecutor,
    pub execute: std::rc::Rc<dyn Fn(&str) -> CommandResult>,
    pub app_config: AppConfigService,
    pub set_theme: Option<std::rc::Rc<dyn Fn(&str) -> bool>>,
}

/// Result of executing a terminal command
/// Commands can either succeed with output or fail with an error message
pub enum CommandResult {
    /// Command executed successfully with the given output
    Success(String),
    /// Command failed with the given error message
    Error(String),
    /// Command executed successfully with HTML output
    Html(String),
    /// Command requires async operation - returns a future
    Async(Pin<Box<dyn Future<Output = CommandResult>>>),
}

/// Trait that all terminal commands must implement
/// This provides a consistent interface for command execution and documentation
pub trait Command {
    /// Execute the command with the given arguments and terminal context
    /// Returns either a success result with output or an error
    fn execute(&self, args: &[String], context: &TerminalContext) -> CommandResult;

    /// Get a brief description of what this command does
    fn description(&self) -> &'static str;

    /// Get usage information showing how to use this command
    fn usage(&self) -> &'static str;

    /// Get detailed help information for this command
    /// Returns None if the command doesn't provide detailed help
    fn help(&self) -> Option<&'static str> {
        None
    }
}