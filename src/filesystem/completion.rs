// Tab Completion for Filesystem
// Handles tab completion for commands and file paths

use super::{navigation::{get_node_at_path, resolve_path}, types::FileSystemNode};

/// Get completion suggestions for tab completion
pub fn get_completion_suggestions(
    root: &FileSystemNode,
    current_path: &[String],
    input: &str,
    cursor_position: usize,
) -> (Vec<String>, String) {
    let input = &input[..cursor_position.min(input.len())];
    let parts: Vec<&str> = input.split_whitespace().collect();

    let mut suggestions = Vec::new();
    let prefix;

    if parts.is_empty() || (parts.len() == 1 && !input.ends_with(' ')) {
        // Complete command names
        let available_commands =
            vec!["ls", "cd", "pwd", "cat", "view", "echo", "help", "clear"];
        let command_prefix = if parts.is_empty() { "" } else { parts[0] };

        for cmd in available_commands {
            if cmd.starts_with(command_prefix) {
                suggestions.push(cmd.to_string());
            }
        }
        prefix = command_prefix.to_string();
    } else {
        // Complete file/directory names
        let last_part = parts.last().unwrap_or(&"");
        let (dir_path, file_prefix) = if last_part.contains('/') {
            let parts: Vec<&str> = last_part.rsplitn(2, '/').collect();
            if parts.len() == 2 {
                (parts[1], parts[0])
            } else {
                ("", *last_part)
            }
        } else {
            ("", *last_part)
        };

        // Resolve directory path
        let target_path = if dir_path.is_empty() {
            current_path.to_vec()
        } else {
            resolve_path(current_path, dir_path)
        };

        // Get suggestions from the target directory
        if let Some(node) = get_node_at_path(root, &target_path) {
            if node.node_type == "directory" {
                for (name, child_node) in &node.children {
                    if name.starts_with(file_prefix) {
                        match child_node.node_type.as_str() {
                            "directory" => suggestions.push(format!("{}/", name)),
                            "file" => suggestions.push(name.clone()),
                            _ => suggestions.push(name.clone()),
                        }
                    }
                }
            }
        }

        prefix = if dir_path.is_empty() {
            file_prefix.to_string()
        } else {
            format!("{}/{}", dir_path, file_prefix)
        };
    }

    suggestions.sort();
    (suggestions, prefix)
}