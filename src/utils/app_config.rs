use serde::{Deserialize, Serialize};
use web_sys::window;

// Include the app.json file content at compile time
const CONFIG_JSON: &str = include_str!("../../app.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Prompt {
    pub symbol: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Terminal {
    pub prompt: Prompt,
    pub background: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Layout {
    pub align: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub author: Author,
    pub terminal: Terminal,
    pub layout: Layout,
    pub theme: String,
}

#[derive(Debug, Clone)]
pub struct AppConfigService {
    pub config: AppConfig,
    pub current_theme: String,
}

impl PartialEq for AppConfigService {
    fn eq(&self, other: &Self) -> bool {
        self.config == other.config && self.current_theme == other.current_theme
    }
}

impl AppConfigService {
    pub fn new() -> Self {
        // Load configuration from embedded JSON at compile time
        let config = Self::load_embedded_config();

        // Check localStorage first, then fall back to config
        let current_theme =
            Self::get_saved_theme().unwrap_or_else(|| Self::resolve_theme(&config.theme));

        Self {
            config,
            current_theme,
        }
    }

    fn load_embedded_config() -> AppConfig {
        // Parse the embedded JSON
        serde_json::from_str::<AppConfig>(CONFIG_JSON).unwrap()
    }

    fn get_saved_theme() -> Option<String> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(theme)) = storage.get_item("terminal-theme") {
                    if matches!(theme.as_str(), "light" | "dark") {
                        return Some(theme);
                    }
                }
            }
        }
        None
    }

    fn save_theme(theme: &str) {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("terminal-theme", theme);
            }
        }
    }

    fn resolve_theme(theme: &str) -> String {
        match theme {
            "system" => {
                // Check if browser supports dark mode preference
                if Self::prefers_dark_mode() {
                    "dark".to_string()
                } else {
                    "light".to_string()
                }
            }
            "light" | "dark" => theme.to_string(),
            _ => "dark".to_string(), // Default to dark
        }
    }

    fn prefers_dark_mode() -> bool {
        // For simplicity, always return true (dark mode preference)
        // In a real implementation, you would use window.matchMedia
        true
    }

    pub fn get_current_theme(&self) -> &str {
        &self.current_theme
    }

    pub fn set_theme(&mut self, theme: &str) -> bool {
        if matches!(theme, "light" | "dark") {
            self.current_theme = theme.to_string();
            Self::save_theme(theme);
            Self::apply_theme_to_dom(theme);
            true
        } else {
            false
        }
    }

    fn apply_theme_to_dom(theme: &str) {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    // Remove existing theme classes
                    let _ = body.set_class_name("");

                    // Add new theme class
                    let theme_class = match theme {
                        "light" => "theme-light",
                        _ => "theme-dark",
                    };
                    let _ = body.set_class_name(theme_class);
                }
            }
        }
    }

    pub fn get_theme_class(&self) -> &str {
        match self.current_theme.as_str() {
            "light" => "theme-light",
            _ => "theme-dark",
        }
    }
}

impl Default for AppConfigService {
    fn default() -> Self {
        Self::new()
    }
}
