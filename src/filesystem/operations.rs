// Filesystem Operations
// File and directory operations on the virtual filesystem

use super::{navigation::{get_node_at_path, resolve_path}, types::FileSystemNode};

/// Read directory contents
pub fn read_directory(
    root: &FileSystemNode,
    current_path: &[String],
    target_dir: Option<&str>,
) -> Result<Vec<String>, String> {
    let target_path = if let Some(target) = target_dir {
        resolve_path(current_path, target)
    } else {
        current_path.to_vec()
    };

    match get_node_at_path(root, &target_path) {
        Some(node) if node.node_type == "directory" => {
            let mut items = Vec::new();
            for (name, child_node) in &node.children {
                match child_node.node_type.as_str() {
                    "directory" => items.push(format!("{}/", name)),
                    "file" => items.push(name.clone()),
                    _ => items.push(name.clone()),
                }
            }
            items.sort();
            Ok(items)
        }
        Some(_) => Err(format!("not a directory: {}", target_dir.unwrap_or("."))),
        None => Err(format!(
            "no such file or directory: {}",
            target_dir.unwrap_or(".")
        )),
    }
}

/// Get directory contents with metadata (for ls --link)
pub fn read_directory_with_metadata<'a>(
    root: &'a FileSystemNode,
    current_path: &[String],
    target_dir: Option<&str>,
) -> Result<Vec<(String, Option<&'a FileSystemNode>)>, String> {
    let target_path = if let Some(target) = target_dir {
        resolve_path(current_path, target)
    } else {
        current_path.to_vec()
    };

    match get_node_at_path(root, &target_path) {
        Some(node) if node.node_type == "directory" => {
            let mut items = Vec::new();
            for (name, child_node) in &node.children {
                let display_name = match child_node.node_type.as_str() {
                    "directory" => format!("{}/", name),
                    "file" => name.clone(),
                    _ => name.clone(),
                };
                items.push((display_name, Some(child_node)));
            }
            items.sort_by_key(|(name, _)| name.clone());
            Ok(items)
        }
        Some(_) => Err(format!("not a directory: {}", target_dir.unwrap_or("."))),
        None => Err(format!(
            "no such file or directory: {}",
            target_dir.unwrap_or(".")
        )),
    }
}

/// Check if a file exists and get its metadata
pub fn get_file_info<'a>(
    root: &'a FileSystemNode,
    current_path: &[String],
    filename: &str,
) -> Result<&'a FileSystemNode, String> {
    let file_path = resolve_path(current_path, filename);
    match get_node_at_path(root, &file_path) {
        Some(node) if node.node_type == "file" => Ok(node),
        Some(_) => Err(format!("is a directory: {}", filename)),
        None => Err(format!("no such file or directory: {}", filename)),
    }
}