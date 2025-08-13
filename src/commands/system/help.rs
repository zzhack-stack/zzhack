// Help Command Implementation
// This file contains the help command that displays information about available commands

use crate::commands::{Command, CommandResult, TerminalContext};

/// Built-in help command that displays information about available commands
/// This command shows a list of all available commands with their descriptions and usage
pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self, _args: &[String], context: &TerminalContext) -> CommandResult {
        let mut help_text = String::from("Available Commands:\n\n");

        // Get all registered commands from the context
        let command_names = context.command_executor.get_command_names();
        let mut sorted_commands = command_names;
        sorted_commands.sort();

        // Add each command with usage and description with proper alignment
        for command_name in &sorted_commands {
            if let Some(command) = context.command_executor.commands.get(command_name) {
                help_text.push_str(&format!(
                    "{:<24}{}\n",
                    command.usage(),
                    command.description()
                ));
            }
        }

        CommandResult::Success(help_text)
    }

    fn description(&self) -> &'static str {
        "Display help information"
    }

    fn usage(&self) -> &'static str {
        "help"
    }
}
