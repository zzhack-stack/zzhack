// Navigate Command Implementation
// This file contains the navigate command that combines history_push and view operations

use super::{Command, CommandResult, TerminalContext};

/// Built-in navigate command that pushes to browser history and views index.md
/// This command combines history_push and view operations for easy navigation
pub struct NavigateCommand;

impl Command for NavigateCommand {
    fn execute(&self, args: &[String], context: &TerminalContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Error("navigate: missing path argument".to_string());
        }

        let path = &args[0];
        let execute = context.execute.clone();

        execute(format!("cd {}", path).as_str());
        execute(format!("history_push {}", path).as_str());
        execute(format!("view {}/index.md", path).as_str())
    }

    fn description(&self) -> &'static str {
        "Navigate to a path and view its index.md"
    }

    fn usage(&self) -> &'static str {
        "navigate <path>"
    }

    fn help(&self) -> Option<&'static str> {
        Some(
            r#"navigate - Navigate to a path and view its index.md

Usage:
  navigate <path>         Navigate to path and view path/index.md
  navigate --help         Show this help message

Description:
  The navigate command performs two operations in sequence:
  1. Pushes the specified path to browser history (using history_push)
  2. Views the index.md file in that path (using view)
  
  This is a convenience command for navigating to different sections
  of a documentation site or file system structure.

Examples:
  navigate about          Push 'about' to history and view 'about/index.md'
  navigate docs/api       Push 'docs/api' to history and view 'docs/api/index.md'
  navigate tutorial       Push 'tutorial' to history and view 'tutorial/index.md'"#,
        )
    }
}
