// Help Command Implementation
// This file contains the help command that displays information about available commands

use super::{Command, CommandResult};

/// Built-in help command that displays information about available commands
/// This command shows a list of all available commands with their descriptions and usage
pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self, _args: &[String]) -> CommandResult {
        let help_text = r#"Available Commands:

help    - Display help information for all commands
echo    - Output the specified text to the terminal
clear   - Clear the terminal screen
pwd     - Print name of current directory
cd      - Change the current directory
cat     - Display file contents

Usage:
  help                    Show all available commands
  echo <text>             Output the specified text
  clear                   Clear all terminal output
  pwd                     Show current directory path
  cd [directory]          Change to directory
  cat <file>              Display contents of file
  echo --red <text>       Output text in red color
  echo --green <text>     Output text in green color
  echo --blue <text>      Output text in blue color
  echo --yellow <text>    Output text in yellow color
  echo --bold <text>      Output text in bold
  echo --rainbow          Display a colorful rainbow text

ANSI Escape Sequences:
  echo "\033[0;31mRed text\033[0m"     Output red text using ANSI codes
  echo "\x1b[1;32mBold green\x1b[0m"   Bold green text using hex escape

Examples:
  echo "Hello World"
  echo --red "Error message"
  echo --green "Success!"
  echo --bold "Important text"
  echo "\033[0;31mLove\033[0m"
  echo "\x1b[1;34mBold Blue\x1b[0m""#;
        
        CommandResult::Success(help_text.to_string())
    }

    fn description(&self) -> &'static str {
        "Display help information"
    }

    fn usage(&self) -> &'static str {
        "help"
    }
}