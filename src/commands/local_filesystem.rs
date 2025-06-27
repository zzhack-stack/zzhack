// Local Filesystem Commands Implementation
// This file contains filesystem commands that use local metadata instead of server calls

use super::{Command, CommandResult};
use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::rc::Rc;

/// PWD Command - Print Working Directory
pub struct PwdCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for PwdCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
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
        CommandResult::Success(fs.pwd())
    }
    
    fn description(&self) -> &'static str {
        "Print name of current directory"
    }
    
    fn usage(&self) -> &'static str {
        "pwd"
    }
}

/// CD Command - Change Directory
pub struct CdCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for CdCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"cd - Change the current directory

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
  cd ..                   Go up one directory level"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        let target = args.first().map(|s| s.as_str()).unwrap_or("");
        let mut fs = self.filesystem.borrow_mut();
        match fs.cd(target) {
            Ok(message) => CommandResult::Success(message),
            Err(error) => CommandResult::Error(error),
        }
    }
    
    fn description(&self) -> &'static str {
        "Change the current directory"
    }
    
    fn usage(&self) -> &'static str {
        "cd [directory]"
    }
}

/// LS Command - List directory contents
pub struct LsCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for LsCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
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
        match fs.ls(target_dir) {
            Ok(output) => CommandResult::Success(output),
            Err(error) => CommandResult::Error(error),
        }
    }
    
    fn description(&self) -> &'static str {
        "List directory contents"
    }
    
    fn usage(&self) -> &'static str {
        "ls [directory]"
    }
}

/// CAT Command - Display file contents
pub struct CatCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for CatCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"cat - Display file contents

Usage:
  cat <file>              Display contents of file
  cat --help              Show this help message

Description:
  The cat command displays the contents of the specified file.
  The file path is relative to the current directory.

Examples:
  cat README.md           Display contents of README.md
  cat about/demo.md       Display contents of about/demo.md"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        if args.is_empty() {
            return CommandResult::Error("cat: missing filename".to_string());
        }
        
        let filename = &args[0];
        let fs = self.filesystem.borrow();
        match fs.cat(filename) {
            Ok(content) => CommandResult::Success(content),
            Err(error) => CommandResult::Error(error),
        }
    }
    
    fn description(&self) -> &'static str {
        "Display file contents"
    }
    
    fn usage(&self) -> &'static str {
        "cat <file>"
    }
}

/// VIEW Command - Render markdown files
pub struct ViewCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for ViewCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
            let help_text = r#"view - Render markdown files

Usage:
  view <file.md>          Render and display markdown file
  view --help             Show this help message

Description:
  The view command renders markdown files and displays them with formatting.
  Only markdown files (.md, .markdown) are supported.
  The output includes colored text, headers, lists, and other markdown elements.

Examples:
  view README.md          Render and display README.md
  view about/demo.md      Render and display about/demo.md"#;
            return CommandResult::Success(help_text.to_string());
        }
        
        if args.is_empty() {
            return CommandResult::Error("view: missing filename".to_string());
        }
        
        let filename = &args[0];
        let fs = self.filesystem.borrow();
        match fs.view(filename) {
            Ok(content) => CommandResult::Success(content),
            Err(error) => CommandResult::Error(error),
        }
    }
    
    fn description(&self) -> &'static str {
        "Render markdown files"
    }
    
    fn usage(&self) -> &'static str {
        "view <file.md>"
    }
}