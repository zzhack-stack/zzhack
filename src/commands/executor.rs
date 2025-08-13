// Command Executor Implementation
// Main command executor that manages and executes terminal commands

use crate::filesystem::FileSystem;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::filesystem::*;
use super::system::*;
use super::types::*;
use super::utility::*;

/// Main command executor that manages and executes terminal commands
///
/// This struct maintains a registry of available commands and handles
/// parsing input strings and routing them to the appropriate command implementations.
pub struct CommandExecutor {
    /// Map of command names to their implementations
    pub commands: HashMap<String, Box<dyn Command>>,
}

impl Clone for CommandExecutor {
    fn clone(&self) -> Self {
        CommandExecutor::new()
    }
}

impl PartialEq for CommandExecutor {
    fn eq(&self, _other: &Self) -> bool {
        // For simplicity, consider all executors equal
        // In practice, you might want to compare command names
        true
    }
}

impl CommandExecutor {
    /// Create a new command executor with all built-in commands registered
    ///
    /// This initializes the executor with the standard set of commands:
    /// - help: Display help information
    /// - echo: Output text to the terminal
    /// - filesystem commands using local metadata
    pub fn new() -> Self {
        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

        // Create shared filesystem instance
        let filesystem = Rc::new(RefCell::new(FileSystem::new()));

        // Register system commands
        commands.insert("help".to_string(), Box::new(HelpCommand));
        commands.insert("whoimi".to_string(), Box::new(WhoimiCommand));
        commands.insert("clear".to_string(), Box::new(ClearCommand));
        commands.insert("theme".to_string(), Box::new(ThemeCommand));
        commands.insert("history_push".to_string(), Box::new(HistoryPushCommand));

        // Register utility commands
        commands.insert("echo".to_string(), Box::new(EchoCommand));
        commands.insert("email".to_string(), Box::new(EmailCommand));
        commands.insert("navigate".to_string(), Box::new(NavigateCommand));
        commands.insert("eval".to_string(), Box::new(EvalCommand {
            filesystem: filesystem.clone(),
        }));

        // Register filesystem commands
        commands.insert("pwd".to_string(), Box::new(PwdCommand {
            filesystem: filesystem.clone(),
        }));
        commands.insert("cd".to_string(), Box::new(CdCommand {
            filesystem: filesystem.clone(),
        }));
        commands.insert("cat".to_string(), Box::new(CatCommand {
            filesystem: filesystem.clone(),
        }));
        commands.insert("ls".to_string(), Box::new(LsCommand {
            filesystem: filesystem.clone(),
        }));
        commands.insert("view".to_string(), Box::new(ViewCommand {
            filesystem: filesystem.clone(),
        }));

        Self { commands }
    }

    /// Get all registered command names
    ///
    /// Returns a vector of all command names that are currently registered
    /// in the command executor. This can be used for syntax highlighting
    /// and command validation.
    pub fn get_command_names(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    /// Execute a command from a raw input string
    ///
    /// This method parses the input string, extracts the command name and arguments,
    /// and routes the execution to the appropriate command implementation.
    ///
    /// # Arguments
    /// * `input` - The raw command string entered by the user
    /// * `context` - Terminal context with utility functions
    ///
    /// # Returns
    /// * `CommandResult` - Either success with output or error with message
    pub fn execute(&self, input: &str, context: &TerminalContext) -> CommandResult {
        let input = input.trim();

        // Handle empty input
        if input.is_empty() {
            return CommandResult::Success(String::new());
        }

        // Parse the input into command and arguments with quote handling
        let parts = Self::parse_command_line(input);

        // Double-check for empty parts
        if parts.is_empty() {
            return CommandResult::Success(String::new());
        }

        // Extract command name and arguments
        let command_name = &parts[0];
        let args = &parts[1..];

        self.execute_command(command_name, args, context)
    }

    /// Execute a specific command with arguments
    pub fn execute_command(
        &self,
        command_name: &str,
        args: &[String],
        context: &TerminalContext,
    ) -> CommandResult {
        // Look up and execute the command
        match self.commands.get(command_name) {
            Some(command) => {
                // Check for --help flag
                if !args.is_empty() && (args[0] == "--help" || args[0] == "-h") {
                    match command.help() {
                        Some(help_text) => CommandResult::Success(help_text.to_string()),
                        None => {
                            // Fallback to basic usage and description if no detailed help
                            CommandResult::Success(format!(
                                "{} - {}\n\nUsage: {}",
                                command_name,
                                command.description(),
                                command.usage()
                            ))
                        }
                    }
                } else {
                    command.execute(args, context)
                }
            }
            None => CommandResult::Error(format!(
                "Unknown command: '{}'. Type 'help' to see available commands.",
                command_name
            )),
        }
    }

    /// Get completion suggestions for tab completion
    ///
    /// This method delegates to the filesystem for file/directory completion
    /// and handles command name completion internally.
    pub fn get_completion_suggestions(
        &self,
        input: &str,
        cursor_position: usize,
    ) -> (Vec<String>, String) {
        // Try to get filesystem from any filesystem command
        if let Some(_pwd_cmd) = self.commands.get("pwd") {
            // We need to access the filesystem through the command
            // For now, create a new filesystem instance for completion
            let fs = FileSystem::new();
            fs.get_completion_suggestions(input, cursor_position)
        } else {
            // Fallback to command completion only
            let input = &input[..cursor_position.min(input.len())];
            let parts: Vec<&str> = input.split_whitespace().collect();

            if parts.is_empty() || (parts.len() == 1 && !input.ends_with(' ')) {
                let command_prefix = if parts.is_empty() { "" } else { parts[0] };
                let mut suggestions = Vec::new();

                for cmd_name in self.get_command_names() {
                    if cmd_name.starts_with(command_prefix) {
                        suggestions.push(cmd_name);
                    }
                }
                suggestions.sort();
                (suggestions, command_prefix.to_string())
            } else {
                (vec![], String::new())
            }
        }
    }

    /// Parse command line input with proper quote handling
    ///
    /// This function handles quoted strings properly, so "hello world" becomes a single argument.
    /// Supports both single and double quotes.
    fn parse_command_line(input: &str) -> Vec<String> {
        let mut parts = Vec::new();
        let mut current_arg = String::new();
        let mut in_quotes = false;
        let mut quote_char = '"';
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '"' | '\'' if !in_quotes => {
                    // Start of quoted string
                    in_quotes = true;
                    quote_char = ch;
                }
                ch if in_quotes && ch == quote_char => {
                    // End of quoted string
                    in_quotes = false;
                }
                ' ' | '\t' if !in_quotes => {
                    // Whitespace outside quotes - end current argument
                    if !current_arg.is_empty() {
                        parts.push(current_arg.clone());
                        current_arg.clear();
                    }
                }
                _ => {
                    // Regular character or whitespace inside quotes
                    current_arg.push(ch);
                }
            }
        }

        // Add the last argument if it's not empty
        if !current_arg.is_empty() {
            parts.push(current_arg);
        }

        parts
    }
}

/// Default implementation for CommandExecutor
impl Default for CommandExecutor {
    fn default() -> Self {
        Self::new()
    }
}