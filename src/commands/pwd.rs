// PWD Command Implementation
// Print Working Directory command

use super::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PwdCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for PwdCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"pwd - Print name of current/working directory

Usage:
  pwd                     Show current directory path
  pwd --help              Show this help message

Description:
  The pwd command displays the full pathname of the current directory.
  The path is shown relative to the project root directory.

Examples:
  pwd"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        let fs = self.filesystem.borrow();
        CommandResult::Success(fs.get_current_directory())
    }
    
    fn description(&self) -> &'static str {
        "Print name of current directory"
    }
    
    fn usage(&self) -> &'static str {
        "pwd"
    }
}