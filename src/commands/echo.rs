// Echo Command Implementation
// This file contains the echo command that outputs text to the terminal

use super::{Command, CommandResult};

/// Built-in echo command that outputs text to the terminal
/// This command takes any number of arguments and outputs them as a single line
pub struct EchoCommand;

impl Command for EchoCommand {
    fn execute(&self, args: &[String]) -> CommandResult {
        if args.is_empty() {
            // Return empty string if no arguments provided
            CommandResult::Success(String::new())
        } else {
            let output = args.join(" ");
            
            // Check for help flag first
            if output == "--help" || output == "-h" {
                let help_text = r#"echo - Output the specified text to the terminal

Usage:
  echo <text>             Output the specified text
  echo --red <text>       Output text in red color
  echo --green <text>     Output text in green color
  echo --blue <text>      Output text in blue color
  echo --yellow <text>    Output text in yellow color
  echo --bold <text>      Output text in bold
  echo --rainbow <text>   Display text in rainbow colors
  echo --rainbow          Display default rainbow text
  echo --help             Show this help message

ANSI Escape Sequences:
  echo "\033[0;31mRed text\033[0m"     Output red text using ANSI codes
  echo "\x1b[1;32mBold green\x1b[0m"   Bold green text using hex escape

Examples:
  echo "Hello World"
  echo --red "Error message"
  echo --green "Success!"
  echo --bold "Important text"
  echo --rainbow "Hello World"
  echo "\033[0;31mLove\033[0m"
  echo "\x1b[1;34mBold Blue\x1b[0m""#;
                return CommandResult::Success(help_text.to_string());
            }
            
            // Check for special color flags
            if output.starts_with("--red ") {
                let text = &output[6..];
                CommandResult::Success(format!("\x1b[31m{}\x1b[0m", text))
            } else if output.starts_with("--green ") {
                let text = &output[8..];
                CommandResult::Success(format!("\x1b[32m{}\x1b[0m", text))
            } else if output.starts_with("--blue ") {
                let text = &output[7..];
                CommandResult::Success(format!("\x1b[34m{}\x1b[0m", text))
            } else if output.starts_with("--yellow ") {
                let text = &output[9..];
                CommandResult::Success(format!("\x1b[33m{}\x1b[0m", text))
            } else if output.starts_with("--bold ") {
                let text = &output[7..];
                CommandResult::Success(format!("\x1b[1m{}\x1b[0m", text))
            } else if output.starts_with("--rainbow ") {
                let text = &output[10..];
                CommandResult::Success(EchoCommand::generate_rainbow_text(text))
            } else if output == "--rainbow" {
                // Fun rainbow text example with default text
                CommandResult::Success(format!(
                    "\x1b[31mR\x1b[33ma\x1b[32mi\x1b[36mn\x1b[34mb\x1b[35mo\x1b[31mw\x1b[0m"
                ))
            } else {
                // Process escape sequences in regular output
                let processed_output = EchoCommand::process_escape_sequences(&output);
                CommandResult::Success(processed_output)
            }
        }
    }

    fn description(&self) -> &'static str {
        "Output the specified text"
    }

    fn usage(&self) -> &'static str {
        "echo <text>"
    }
}

impl EchoCommand {
    /// Generate rainbow colored text by cycling through colors for each character
    fn generate_rainbow_text(text: &str) -> String {
        let colors = [
            "\x1b[31m", // Red
            "\x1b[33m", // Yellow  
            "\x1b[32m", // Green
            "\x1b[36m", // Cyan
            "\x1b[34m", // Blue
            "\x1b[35m", // Magenta
        ];
        
        let mut result = String::new();
        let mut color_index = 0;
        
        for ch in text.chars() {
            if ch != ' ' {
                // Apply color to non-space characters
                result.push_str(colors[color_index % colors.len()]);
                result.push(ch);
                color_index += 1;
            } else {
                // Keep spaces as-is
                result.push(ch);
            }
        }
        
        // Reset color at the end
        result.push_str("\x1b[0m");
        result
    }

    /// Process escape sequences like \033, \x1b, \\n, \\t etc.
    fn process_escape_sequences(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.peek() {
                    Some('0') => {
                        chars.next(); // consume '0'
                        if chars.peek() == Some(&'3') {
                            chars.next(); // consume '3'
                            if chars.peek() == Some(&'3') {
                                chars.next(); // consume '3'
                                // \033 -> ESC character
                                result.push('\x1b');
                            } else {
                                // Not \033, put back the characters
                                result.push('\\');
                                result.push('0');
                                result.push('3');
                            }
                        } else {
                            // Not \033, put back the characters
                            result.push('\\');
                            result.push('0');
                        }
                    }
                    Some('x') => {
                        chars.next(); // consume 'x'
                        if chars.peek() == Some(&'1') {
                            chars.next(); // consume '1'
                            if chars.peek() == Some(&'b') {
                                chars.next(); // consume 'b'
                                // \x1b -> ESC character
                                result.push('\x1b');
                            } else {
                                // Not \x1b, put back the characters
                                result.push('\\');
                                result.push('x');
                                result.push('1');
                            }
                        } else {
                            // Not \x1b, put back the characters
                            result.push('\\');
                            result.push('x');
                        }
                    }
                    Some('n') => {
                        chars.next(); // consume 'n'
                        result.push('\n');
                    }
                    Some('t') => {
                        chars.next(); // consume 't'
                        result.push('\t');
                    }
                    Some('r') => {
                        chars.next(); // consume 'r'
                        result.push('\r');
                    }
                    Some('\\') => {
                        chars.next(); // consume '\\'
                        result.push('\\');
                    }
                    _ => {
                        // Unknown escape sequence, keep the backslash
                        result.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        }
        
        result
    }
}