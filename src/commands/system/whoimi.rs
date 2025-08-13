use crate::commands::{Command, CommandResult, TerminalContext};

pub struct WhoimiCommand;

impl Command for WhoimiCommand {
    fn execute(&self, _args: &[String], context: &TerminalContext) -> CommandResult {
        let execute = context.execute.clone();

        execute(format!("echo {}", context.app_config.config.author.name).as_str())
    }

    fn description(&self) -> &'static str {
        "Output the author's name"
    }

    fn usage(&self) -> &'static str {
        "whoami"
    }
}
