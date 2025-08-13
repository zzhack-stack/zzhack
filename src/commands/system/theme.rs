use crate::commands::{Command, CommandResult, TerminalContext};

pub struct ThemeCommand;

impl Command for ThemeCommand {
    fn execute(&self, args: &[String], context: &TerminalContext) -> CommandResult {
        if args.is_empty() {
            // Display current theme
            let current_theme = context.app_config.get_current_theme();
            let config_theme = &context.app_config.config.theme;
            
            if config_theme == "system" {
                CommandResult::Success(format!(
                    "Current theme: {} (resolved from system preference)\nDefault theme: {}",
                    current_theme, config_theme
                ))
            } else {
                CommandResult::Success(format!("Current theme: {}", current_theme))
            }
        } else if args.len() == 2 && args[0] == "switch" {
            // Switch theme
            let new_theme = &args[1];
            
            if matches!(new_theme.as_str(), "light" | "dark") {
                if let Some(set_theme) = &context.set_theme {
                    if set_theme(new_theme) {
                        CommandResult::Success(format!("Theme switched to: {}", new_theme))
                    } else {
                        CommandResult::Error("Failed to switch theme".to_string())
                    }
                } else {
                    CommandResult::Error("Theme switching not available in this context".to_string())
                }
            } else {
                CommandResult::Error(format!(
                    "Invalid theme: '{}'. Available themes: light, dark", 
                    new_theme
                ))
            }
        } else {
            CommandResult::Error("Invalid usage. Use 'theme' to show current theme or 'theme switch <light|dark>' to switch theme.".to_string())
        }
    }

    fn description(&self) -> &'static str {
        "Display or switch the current theme"
    }

    fn usage(&self) -> &'static str {
        "theme [switch <light|dark>]"
    }

    fn help(&self) -> Option<&'static str> {
        Some(
            "Theme command usage:
  theme              - Display current theme
  theme switch light - Switch to light theme
  theme switch dark  - Switch to dark theme

The default theme is configured in app.json and can be:
- light: Always use light theme
- dark: Always use dark theme  
- system: Use system preference (prefers-color-scheme)"
        )
    }
}