// Navigation Handlers
// Handlers for command history navigation (arrow up/down)

use yew::prelude::*;

/// Handle arrow up key for command history navigation
pub fn handle_arrow_up(
    command_history: &UseStateHandle<Vec<String>>,
    history_index: &UseStateHandle<Option<usize>>,
    input_value: &UseStateHandle<String>,
    cursor_position: &UseStateHandle<usize>,
) {
    let cmd_history = &**command_history;
    if !cmd_history.is_empty() {
        let new_index = match **history_index {
            None => cmd_history.len() - 1,
            Some(idx) => {
                if idx > 0 {
                    idx - 1
                } else {
                    0
                }
            }
        };
        history_index.set(Some(new_index));
        input_value.set(cmd_history[new_index].clone());
        cursor_position.set(cmd_history[new_index].len());
    }
}

/// Handle arrow down key for command history navigation
pub fn handle_arrow_down(
    command_history: &UseStateHandle<Vec<String>>,
    history_index: &UseStateHandle<Option<usize>>,
    input_value: &UseStateHandle<String>,
    cursor_position: &UseStateHandle<usize>,
) {
    let cmd_history = &**command_history;
    if !cmd_history.is_empty() {
        match **history_index {
            None => {}
            Some(idx) => {
                if idx < cmd_history.len() - 1 {
                    let new_index = idx + 1;
                    history_index.set(Some(new_index));
                    input_value.set(cmd_history[new_index].clone());
                    cursor_position.set(cmd_history[new_index].len());
                } else {
                    history_index.set(None);
                    input_value.set(String::new());
                    cursor_position.set(0);
                }
            }
        }
    }
}