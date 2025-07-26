use serde::{Deserialize, Serialize};
use yew::prelude::*;

// Include the app.json file content at compile time
const CONFIG_JSON: &str = include_str!("../../app.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Terminal {
    pub prompt: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub author: Author,
    pub terminal: Terminal,
}

#[derive(Debug, Clone)]
pub struct AppConfigService {
    pub config: AppConfig,
}

impl PartialEq for AppConfigService {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config
    }
}

impl AppConfigService {
    pub fn new() -> Self {
        // Load configuration from embedded JSON at compile time
        let config = Self::load_embedded_config();
        Self { config }
    }

    fn load_embedded_config() -> AppConfig {
        // Parse the embedded JSON
        serde_json::from_str::<AppConfig>(CONFIG_JSON).unwrap()
    }
}

impl Default for AppConfigService {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook for using AppConfig in Yew components
#[hook]
pub fn use_app_config() -> UseStateHandle<AppConfigService> {
    use_state(|| AppConfigService::new())
}
