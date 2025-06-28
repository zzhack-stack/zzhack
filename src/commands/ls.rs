// LS Command Implementation
// List directory contents command

use super::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LsCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for LsCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"ls - List directory contents

Usage:
  ls [directory]          List contents of directory
  ls                      List contents of current directory
  ls --help               Show this help message

Description:
  The ls command lists the contents of the specified directory.
  If no directory is specified, lists the current directory.
  All paths are relative to the current working directory.

Examples:
  ls                      List current directory contents
  ls about                List contents of about directory
  ls links/demo           List contents of links/demo directory"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        let target_dir = args.first().map(|s| s.as_str());
        let fs = self.filesystem.borrow();
        
        fs.read_directory(target_dir)
            .map(|items| {
                let output = if items.is_empty() {
                    String::new()
                } else {
                    items.join("  ")
                };
                CommandResult::Success(output)
            })
            .unwrap_or_else(|error| CommandResult::Error(format!("ls: {}", error)))
    }
    
    fn description(&self) -> &'static str {
        "List directory contents"
    }
    
    fn usage(&self) -> &'static str {
        "ls [directory]"
    }
}