// Clear Command Implementation
// This file contains the clear command that clears the terminal screen

use crate::commands::{Command, CommandResult, TerminalContext};

/// Built-in clear command that clears the terminal screen
/// This command removes all previous output and resets the terminal to a clean state
pub struct ClearCommand;

impl Command for ClearCommand {
    fn execute(&self, _args: &[String], context: &TerminalContext) -> CommandResult {
        // Call the terminal's clear screen function directly
        (context.clear_screen)();
        CommandResult::Success(String::new())
    }

    fn description(&self) -> &'static str {
        "Clear the terminal screen"
    }

    fn usage(&self) -> &'static str {
        "clear"
    }

    fn help(&self) -> Option<&'static str> {
        Some(r#"clear - Clear the terminal screen

Usage:
  clear                   Clear all terminal output
  clear --help            Show this help message

Description:
  The clear command removes all previous command output and history
  from the terminal display, providing a clean workspace.

Examples:
  clear"#)
    }
}