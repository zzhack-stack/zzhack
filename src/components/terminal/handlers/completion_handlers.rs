// Tab Completion Handlers
// Handles tab completion for commands and file paths

use crate::commands::CommandExecutor;
use yew::prelude::*;

/// Handle tab completion
pub fn handle_tab(
    executor: &UseStateHandle<CommandExecutor>,
    input_value: &UseStateHandle<String>,
    cursor_position: &UseStateHandle<usize>,
) {
    let (suggestions, prefix) =
        executor.get_completion_suggestions(&**input_value, **cursor_position);

    if !suggestions.is_empty() {
        let suggestion = &suggestions[0];
        let current_input = (**input_value).clone();
        let cursor_pos = **cursor_position;

        let prefix_start = if cursor_pos >= prefix.len() {
            cursor_pos - prefix.len()
        } else {
            0
        };

        let mut new_input = String::new();
        new_input.push_str(&current_input[..prefix_start]);
        new_input.push_str(suggestion);
        new_input.push_str(&current_input[cursor_pos..]);

        let new_cursor_pos = prefix_start + suggestion.len();

        input_value.set(new_input);
        cursor_position.set(new_cursor_pos);
    }
}