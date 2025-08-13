// Filesystem Types
// Core data structures for the virtual filesystem

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    // Markdown metadata fields
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}