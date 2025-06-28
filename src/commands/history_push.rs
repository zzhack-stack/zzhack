// History Push Command Implementation
// This file contains the history_push command that manipulates browser history

use super::{Command, CommandResult, TerminalContext};
use crate::utils::config::get_base_url;
use web_sys::window;

/// Built-in history_push command that pushes a new path to browser history
/// This command allows navigating the browser URL without page reload
pub struct HistoryPushCommand;

impl Command for HistoryPushCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Error("history_push: missing path argument".to_string());
        }

        let path = &args[0];
        
        // Handle baseurl - prepend baseurl to the path
        let baseurl = get_base_url();
        let full_path = if path.starts_with('/') {
            // Path starts with /, append to baseurl
            format!("{}{}", baseurl, path)
        } else {
            // Path doesn't start with /, append with separator
            format!("{}/{}", baseurl, path)
        };
        
        // Get the window object and history API
        match window() {
            Some(window) => {
                match window.history() {
                    Ok(history) => {
                        // Push the new path to browser history
                        match history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&full_path)) {
                            Ok(_) => CommandResult::Success(format!("Pushed '{}' to browser history", full_path)),
                            Err(_) => CommandResult::Error("Failed to push to browser history".to_string()),
                        }
                    }
                    Err(_) => CommandResult::Error("Failed to access browser history API".to_string()),
                }
            }
            None => CommandResult::Error("Failed to access window object".to_string()),
        }
    }

    fn description(&self) -> &'static str {
        "Push a path to browser history"
    }

    fn usage(&self) -> &'static str {
        "history_push <path>"
    }

    fn help(&self) -> Option<&'static str> {
        Some(r#"history_push - Push a path to browser history

Usage:
  history_push <path>     Push the specified path to browser history
  history_push --help     Show this help message

Description:
  The history_push command uses the browser's History API to push a new
  path to the browser's navigation history. This changes the URL in the
  address bar without triggering a page reload.

Examples:
  history_push /about     Push '/about' to browser history
  history_push a/b        Push 'a/b' to browser history
  history_push /docs/api  Push '/docs/api' to browser history"#)
    }
}