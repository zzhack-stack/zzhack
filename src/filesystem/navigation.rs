// Filesystem Navigation
// Path resolution and navigation utilities

use super::types::FileSystemNode;

/// Resolve a relative or absolute path from current location
pub fn resolve_path(current_path: &[String], target: &str) -> Vec<String> {
    if target.starts_with('/') {
        // Absolute path
        if target == "/" {
            vec![]
        } else {
            target
                .trim_start_matches('/')
                .split('/')
                .map(|s| s.to_string())
                .collect()
        }
    } else if target == ".." {
        // Go up one directory
        if current_path.is_empty() {
            vec![]
        } else {
            let mut new_path = current_path.to_vec();
            new_path.pop();
            new_path
        }
    } else if target == "." {
        // Current directory
        current_path.to_vec()
    } else {
        // Relative path
        let mut new_path = current_path.to_vec();
        for component in target.split('/') {
            if component == ".." {
                new_path.pop();
            } else if component != "." && !component.is_empty() {
                new_path.push(component.to_string());
            }
        }
        new_path
    }
}

/// Navigate to a node at the given path
pub fn get_node_at_path<'a>(root: &'a FileSystemNode, path: &[String]) -> Option<&'a FileSystemNode> {
    let mut current = root;

    for component in path {
        if current.node_type != "directory" {
            return None; // Can't navigate into a file
        }
        current = current.children.get(component)?;
    }

    Some(current)
}

/// Get current directory path as string
pub fn path_to_string(path: &[String]) -> String {
    if path.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", path.join("/"))
    }
}