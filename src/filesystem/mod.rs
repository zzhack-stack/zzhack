// Local Filesystem Module
// This module provides filesystem operations using pre-generated metadata
// instead of a server-based virtual filesystem

use crate::utils::config::build_data_url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileSystemNode {
    #[serde(rename = "type")]
    pub node_type: String,
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub children: HashMap<String, FileSystemNode>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub modified: Option<String>,
    #[serde(default)]
    pub extension: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    root: FileSystemNode,
    current_path: Vec<String>,
}

impl FileSystem {
    /// Create a new filesystem from the embedded metadata
    pub fn new() -> Self {
        let metadata_json = include_str!("../filesystem_metadata.json");
        let root: FileSystemNode =
            serde_json::from_str(metadata_json).expect("Failed to parse filesystem metadata");

        Self {
            root,
            current_path: vec![],
        }
    }

    /// Get current directory path as string
    pub fn current_path_string(&self) -> String {
        if self.current_path.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", self.current_path.join("/"))
        }
    }

    /// Navigate to a path and get the node
    fn get_node_at_path(&self, path: &[String]) -> Option<&FileSystemNode> {
        let mut current = &self.root;

        for component in path {
            if current.node_type != "directory" {
                return None; // Can't navigate into a file
            }
            current = current.children.get(component)?;
        }

        Some(current)
    }

    /// Get node at current directory
    fn get_current_node(&self) -> Option<&FileSystemNode> {
        self.get_node_at_path(&self.current_path)
    }

    /// Resolve a relative or absolute path from current location
    fn resolve_path(&self, target: &str) -> Vec<String> {
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
            if self.current_path.is_empty() {
                vec![]
            } else {
                let mut new_path = self.current_path.clone();
                new_path.pop();
                new_path
            }
        } else if target == "." {
            // Current directory
            self.current_path.clone()
        } else {
            // Relative path
            let mut new_path = self.current_path.clone();
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

    /// Get current working directory path
    pub fn get_current_directory(&self) -> String {
        self.current_path_string()
    }

    /// Navigate to a directory
    pub fn navigate(&mut self, target: &str) -> Result<(), String> {
        let target = if target.is_empty() { "/" } else { target };
        let new_path = self.resolve_path(target);

        match self.get_node_at_path(&new_path) {
            Some(node) if node.node_type == "directory" => {
                self.current_path = new_path;
                Ok(())
            }
            Some(_) => Err(format!("not a directory: {}", target)),
            None => Err(format!("no such file or directory: {}", target)),
        }
    }

    /// Read directory contents
    pub fn read_directory(&self, target_dir: Option<&str>) -> Result<Vec<String>, String> {
        let target_path = if let Some(target) = target_dir {
            self.resolve_path(target)
        } else {
            self.current_path.clone()
        };

        match self.get_node_at_path(&target_path) {
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
            None => Err(format!("no such file or directory: {}", target_dir.unwrap_or("."))),
        }
    }

    /// Check if a file exists and get its metadata
    pub fn get_file_info(&self, filename: &str) -> Result<&FileSystemNode, String> {
        let file_path = self.resolve_path(filename);
        match self.get_node_at_path(&file_path) {
            Some(node) if node.node_type == "file" => Ok(node),
            Some(_) => Err(format!("is a directory: {}", filename)),
            None => Err(format!("no such file or directory: {}", filename)),
        }
    }

    /// Check if a path exists and is a directory
    pub fn is_directory(&self, path: &str) -> bool {
        let resolved_path = self.resolve_path(path);
        matches!(self.get_node_at_path(&resolved_path), Some(node) if node.node_type == "directory")
    }

    /// Check if a path exists and is a file
    pub fn is_file(&self, path: &str) -> bool {
        let resolved_path = self.resolve_path(path);
        matches!(self.get_node_at_path(&resolved_path), Some(node) if node.node_type == "file")
    }

    /// Get completion suggestions for tab completion
    pub fn get_completion_suggestions(
        &self,
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
                self.current_path.clone()
            } else {
                self.resolve_path(dir_path)
            };

            // Get suggestions from the target directory
            if let Some(node) = self.get_node_at_path(&target_path) {
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
}

/// Fetch file content from the data directory
pub async fn fetch_file_content(file_path: &str) -> Result<String, String> {
    let url = build_data_url(file_path);

    let mut opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| format!("Failed to create request for {}", file_path))?;

    let window = web_sys::window().ok_or("No window object")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| format!("Network request failed for {}", file_path))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Failed to cast response".to_string())?;

    if !resp.ok() {
        return Err(format!(
            "Failed to fetch file {}: HTTP {}",
            file_path,
            resp.status()
        ));
    }

    let text_promise = resp.text().map_err(|_| "Failed to get text promise")?;

    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|_| "Failed to get text from response")?;

    let content = text_value
        .as_string()
        .ok_or("Failed to convert response to string")?;

    Ok(content)
}

/// Check if file should be syntax highlighted
fn should_highlight_file(filename: &str) -> bool {
    if let Some(ext) = filename.split('.').last() {
        matches!(
            ext.to_lowercase().as_str(),
            "rs" | "js"
                | "ts"
                | "jsx"
                | "tsx"
                | "py"
                | "java"
                | "cpp"
                | "c"
                | "h"
                | "css"
                | "scss"
                | "html"
                | "xml"
                | "json"
                | "yaml"
                | "yml"
                | "toml"
                | "go"
                | "php"
                | "rb"
                | "swift"
                | "kt"
                | "cs"
                | "sh"
                | "bash"
                | "zsh"
                | "fish"
                | "ps1"
                | "sql"
                | "r"
                | "scala"
                | "clj"
                | "hs"
                | "elm"
                | "dart"
                | "vue"
                | "svelte"
                | "tex"
                | "dockerfile"
                | "makefile"
                | "gradle"
        )
    } else {
        false
    }
}

/// Apply basic syntax highlighting (simplified)
fn apply_syntax_highlighting(content: &str, filename: &str) -> String {
    // For now, just return the content as-is
    // In a real implementation, you would parse and highlight the syntax
    content.to_string()
}
