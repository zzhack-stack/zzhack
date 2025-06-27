// Clear Command Implementation
// This file contains the clear command that clears the terminal screen

use super::{Command, CommandResult};

/// Built-in clear command that clears the terminal screen
/// This command removes all previous output and resets the terminal to a clean state
pub struct ClearCommand;

impl Command for ClearCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        // Check for help flag first
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"clear - Clear the terminal screen

Usage:
  clear                   Clear all terminal output
  clear --help            Show this help message

Description:
  The clear command removes all previous command output and history
  from the terminal display, providing a clean workspace.

Examples:
  clear"#;
            return CommandResult::Success(help_text.to_string());
        }

        // Clear command returns a special marker that the terminal will recognize
        // We use a special control sequence to indicate screen clearing
        CommandResult::Success("__CLEAR_SCREEN__".to_string())
    }

    fn description(&self) -> &'static str {
        "Clear the terminal screen"
    }

    fn usage(&self) -> &'static str {
        "clear"
    }
}