// CD Command Implementation
// Change Directory command

use crate::commands::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CdCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for CdCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
        let target = args.first().map(|s| s.as_str()).unwrap_or("");
        let mut fs = self.filesystem.borrow_mut();
        
        fs.navigate(target)
            .map(|_| CommandResult::Success(format!(
                "Changed directory to {}",
                fs.get_current_directory()
            )))
            .unwrap_or_else(|error| CommandResult::Error(format!("cd: {}", error)))
    }
    
    fn description(&self) -> &'static str {
        "Change the current directory"
    }
    
    fn usage(&self) -> &'static str {
        "cd [directory]"
    }

    fn help(&self) -> Option<&'static str> {
        Some(r#"cd - Change the current directory

Usage:
  cd [directory]          Change to directory
  cd                      Change to root directory  
  cd --help               Show this help message

Description:
  The cd command changes the current working directory to the specified
  directory. If no directory is specified, changes to the root directory.
  All paths are relative to the project root directory.

Examples:
  cd                      Change to root directory
  cd about                Change to about directory
  cd about/demo           Change to about/demo directory
  cd ..                   Go up one directory level"#)
    }
}