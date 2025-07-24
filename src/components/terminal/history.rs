// Terminal History Component
// Renders command history entries

use crate::commands::CommandExecutor;
use crate::components::history::{HistoryEntry, HistoryItem};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalHistoryProps {
    pub history: Vec<HistoryEntry>,
    pub executor: CommandExecutor,
}

#[function_component(TerminalHistory)]
pub fn terminal_history(props: &TerminalHistoryProps) -> Html {
    let valid_commands = props.executor.get_command_names();
    
    html! {
        <>
            {for props.history.iter().map(|entry| {
                html! {
                    <HistoryItem 
                        entry={entry.clone()} 
                        valid_commands={valid_commands.clone()}
                    />
                }
            })}
        </>
    }
}