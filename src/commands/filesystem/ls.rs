// LS Command Implementation
// List directory contents command

use crate::commands::{Command, CommandResult, TerminalContext};
use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::rc::Rc;

pub struct LsCommand {
    pub filesystem: Rc<RefCell<FileSystem>>,
}

impl Command for LsCommand {
    fn execute(&self, args: &[String], _context: &TerminalContext) -> CommandResult {
        let fs = self.filesystem.borrow();

        // Check for --link option
        let use_link_format = args.contains(&"--link".to_string());
        let target_dir = if use_link_format {
            // Find directory argument (not --link)
            args.iter().find(|&arg| arg != "--link").map(|s| s.as_str())
        } else {
            args.first().map(|s| s.as_str())
        };

        if use_link_format {
            Self::execute_link_format(&*fs, target_dir)
        } else {
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
    }

    fn description(&self) -> &'static str {
        "List directory contents"
    }

    fn usage(&self) -> &'static str {
        "ls [directory] [--link]"
    }

    fn help(&self) -> Option<&'static str> {
        Some(
            r#"ls - List directory contents

Usage:
  ls [directory]          List contents of directory
  ls --link [directory]   List with detailed format showing title, description, and modified date
  ls                      List contents of current directory
  ls --help               Show this help message

Description:
  The ls command lists the contents of the specified directory.
  If no directory is specified, lists the current directory.
  All paths are relative to the current working directory.
  
  The --link option shows a detailed vertical format with:
  - Icon (📁 for directories, 📄 for markdown files)
  - Title (from metadata or filename)
  - Description (from metadata if available)
  - Modified date

Examples:
  ls                      List current directory contents
  ls about                List contents of about directory
  ls --link               List current directory with detailed format
  ls --link about         List about directory with detailed format"#,
        )
    }
}

impl LsCommand {
    fn execute_link_format(fs: &FileSystem, target_dir: Option<&str>) -> CommandResult {
        // Use the new method that provides direct access to metadata
        match fs.read_directory_with_metadata(target_dir) {
            Ok(items) => {
                let mut output_lines = Vec::new();

                // No headers or separators - clean output

                // If no items, show an empty result message
                if items.is_empty() {
                    let html = r#"<div class="file-list-empty">
                        <p>(empty directory)</p>
                    </div>"#;
                    return CommandResult::Html(html.to_string());
                }

                // Start building HTML
                output_lines.push("<div class=\"file-list\">".to_string());
                
                for (item_name, node_opt) in items {
                    let item_name_clean = item_name.trim_end_matches('/');
                    let is_directory = item_name.ends_with('/');

                    let (icon, title, description, modified) = if let Some(node) = node_opt {
                        if is_directory {
                            // For directories, try to find index.md in children
                            if let Some(index_node) = node.children.get("index.md") {
                                (
                                    Self::get_folder_svg(),
                                    index_node.title.as_deref().unwrap_or(item_name_clean),
                                    index_node.description.as_deref().unwrap_or("-"),
                                    index_node.modified.as_deref().unwrap_or("-"),
                                )
                            } else {
                                (Self::get_folder_svg(), item_name_clean, "-", "-")
                            }
                        } else {
                            // Regular file - use its metadata directly
                            let icon = if item_name_clean.ends_with(".md") {
                                Self::get_markdown_svg()
                            } else {
                                Self::get_file_svg()
                            };
                            (
                                icon,
                                node.title.as_deref().unwrap_or(item_name_clean),
                                node.description.as_deref().unwrap_or("-"),
                                node.modified.as_deref().unwrap_or("-"),
                            )
                        }
                    } else {
                        // Fallback if no metadata
                        let icon = if is_directory {
                            Self::get_folder_svg()
                        } else if item_name_clean.ends_with(".md") {
                            Self::get_markdown_svg()
                        } else {
                            Self::get_file_svg()
                        };
                        (icon, item_name_clean, "-", "-")
                    };

                    // Determine click action based on file type
                    let click_action = if is_directory {
                        format!("navigate {}", item_name_clean)
                    } else if item_name_clean.ends_with(".md") {
                        format!("view {}", item_name_clean)
                    } else {
                        format!("cat {}", item_name_clean)
                    };
                    
                    let formatted_modified = Self::format_date(modified);
                    let truncated_title = Self::truncate(title, 40);
                    let truncated_description = Self::truncate(description, 60);
                    
                    // Create clickable HTML list item
                    let html_item = format!(
                        r#"<div class="file-item" onclick="window.executeCommand('{}')">
                            <span class="file-icon">{}</span>
                            <div class="file-info">
                                <div class="file-title">{}</div>
                                <div class="file-description">{}</div>
                            </div>
                            <div class="file-modified">{}</div>
                        </div>"#,
                        click_action,
                        icon,
                        truncated_title,
                        truncated_description,
                        formatted_modified
                    );
                    
                    output_lines.push(html_item);
                }
                
                output_lines.push("</div>".to_string());

                CommandResult::Html(output_lines.join("\n"))
            }
            Err(error) => CommandResult::Error(format!("ls --link error: {}", error)),
        }
    }

    fn truncate(text: &str, max_len: usize) -> String {
        if text.chars().count() <= max_len {
            text.to_string()
        } else {
            let mut result = String::new();
            let mut char_count = 0;
            
            for ch in text.chars() {
                if char_count + 4 > max_len { // Reserve 3 chars for "..." plus safety margin
                    break;
                }
                result.push(ch);
                char_count += 1;
            }
            
            format!("{}...", result)
        }
    }

    fn format_date(date_str: &str) -> String {
        if date_str == "-" {
            return "-".to_string();
        }

        // Parse ISO date format and show just the date part
        if let Some(date_part) = date_str.split('T').next() {
            date_part.to_string()
        } else {
            date_str.to_string()
        }
    }
    
    fn get_folder_svg() -> String {
        format!(
            r#"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>"#,
            "#32cd32"
        )
    }
    
    fn get_markdown_svg() -> String {
        format!(
            r#"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14,2 14,8 20,8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <polyline points="10,9 9,9 8,9"/>
        </svg>"#,
            "#39ff14"
        )
    }
    
    fn get_file_svg() -> String {
        format!(
            r#"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="{}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14,2 14,8 20,8"/>
        </svg>"#,
            "#00ff7f"
        )
    }
}
