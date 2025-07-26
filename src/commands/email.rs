use super::Command;

pub struct EmailCommand;

impl Command for EmailCommand {
    fn execute(&self, _args: &[String], context: &super::TerminalContext) -> super::CommandResult {
        let execute = context.execute.clone();

        execute(format!("echo {}", context.app_config.config.author.email).as_str())
    }

    fn description(&self) -> &'static str {
        "Output the author's email"
    }

    fn usage(&self) -> &'static str {
        "email"
    }
}
