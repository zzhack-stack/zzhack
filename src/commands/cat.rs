// CAT Command Implementation
// Display file contents command

use super::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use crate::utils::fetch_file_content;
use std::cell::RefCell;
use std::rc::Rc;

pub struct CatCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for CatCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
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

        match fs.get_file_info(filename) {
            Ok(node) => {
                let file_path = node.path.clone();

                // Return async future that will resolve to the file content
                let future = Box::pin(async move {
                    match fetch_file_content(&file_path).await {
                        Ok(content) => {
                            // Output file content as plain text (manually escape HTML)
                            let escaped_content = content
                                .replace('&', "&amp;")
                                .replace('<', "&lt;")
                                .replace('>', "&gt;")
                                .replace('"', "&quot;")
                                .replace('\'', "&#x27;");
                            let html_content =
                                format!("<pre class=\"file-content\">{}</pre>", escaped_content);
                            CommandResult::Html(html_content)
                        }
                        Err(error) => {
                            CommandResult::Error(format!("cat: Error reading file: {}", error))
                        }
                    }
                });

                CommandResult::Async(future)
            }
            Err(error) => CommandResult::Error(format!("cat: {}", error)),
        }
    }

    fn description(&self) -> &'static str {
        "Display file contents"
    }

    fn usage(&self) -> &'static str {
        "cat <file>"
    }
}
