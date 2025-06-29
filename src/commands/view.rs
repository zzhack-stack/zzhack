// VIEW Command Implementation
// Render markdown files command

use super::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use crate::utils::fetch_and_render_markdown_with_executor;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ViewCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for ViewCommand {
    fn execute(&self, args: &[String], context: &TerminalContext) -> CommandResult {
        if args.is_empty() {
            return CommandResult::Error("view: missing filename".to_string());
        }
        
        let filename = &args[0];
        let fs = self.filesystem.borrow();
        
        match fs.get_file_info(filename) {
            Ok(node) => {
                // Check if it's a markdown file using the extension field
                let is_markdown = node
                    .extension
                    .as_ref()
                    .map(|ext| ext == "md" || ext == "markdown")
                    .unwrap_or(false);

                if !is_markdown {
                    CommandResult::Error(format!("view: not a markdown file: {}", filename))
                } else {
                    let file_path = node.path.clone();
                    
                    // Return async future that will resolve to the rendered markdown with command execution support
                    let executor = context.command_executor.clone();
                    let future = Box::pin(async move {
                        match fetch_and_render_markdown_with_executor(&file_path, &executor).await {
                            Ok(html_content) => {
                                CommandResult::Html(html_content)
                            }
                            Err(error) => {
                                CommandResult::Error(format!("view: Error rendering markdown: {}", error))
                            }
                        }
                    });
                    
                    CommandResult::Async(future)
                }
            }
            Err(error) => CommandResult::Error(format!("view: {}", error))
        }
    }
    
    fn description(&self) -> &'static str {
        "Render markdown files"
    }
    
    fn usage(&self) -> &'static str {
        "view <file.md>"
    }

    fn help(&self) -> Option<&'static str> {
        Some(r#"view - Render markdown files

Usage:
  view <file.md>          Render and display markdown file
  view --help             Show this help message

Description:
  The view command renders markdown files and displays them with formatting.
  Only markdown files (.md, .markdown) are supported.
  The output includes styled HTML with colors, headers, lists, and other markdown elements.

Examples:
  view README.md          Render and display README.md
  view about/demo.md      Render and display about/demo.md"#)
    }
}