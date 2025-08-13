// FileSystem Implementation
// Main filesystem struct with high-level operations

use super::{
    completion::get_completion_suggestions as get_fs_completion,
    navigation::{get_node_at_path, path_to_string, resolve_path},
    operations::{get_file_info as get_fs_file_info, read_directory as read_fs_directory, read_directory_with_metadata as read_fs_directory_with_metadata},
    types::FileSystemNode,
};

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
        path_to_string(&self.current_path)
    }

    /// Get current working directory path
    pub fn get_current_directory(&self) -> String {
        self.current_path_string()
    }

    /// Navigate to a directory
    pub fn navigate(&mut self, target: &str) -> Result<(), String> {
        let target = if target.is_empty() { "/" } else { target };
        let new_path = resolve_path(&self.current_path, target);

        match get_node_at_path(&self.root, &new_path) {
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
        read_fs_directory(&self.root, &self.current_path, target_dir)
    }

    /// Check if a file exists and get its metadata
    pub fn get_file_info(&self, filename: &str) -> Result<&FileSystemNode, String> {
        get_fs_file_info(&self.root, &self.current_path, filename)
    }

    /// Get directory contents with metadata (for ls --link)
    pub fn read_directory_with_metadata(
        &self,
        target_dir: Option<&str>,
    ) -> Result<Vec<(String, Option<&FileSystemNode>)>, String> {
        read_fs_directory_with_metadata(&self.root, &self.current_path, target_dir)
    }

    /// Get completion suggestions for tab completion
    pub fn get_completion_suggestions(
        &self,
        input: &str,
        cursor_position: usize,
    ) -> (Vec<String>, String) {
        get_fs_completion(&self.root, &self.current_path, input, cursor_position)
    }
}