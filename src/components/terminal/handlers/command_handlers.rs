// Command Execution Handlers
// Handlers for command execution and result processing

use crate::commands::{CommandExecutor, CommandResult, TerminalContext};
use crate::components::history::{
    create_command_entry, create_html_entry, create_welcome_entry, HistoryEntry,
};
use crate::utils::AppConfigService;
use yew::prelude::*;

use super::{handle_arrow_down, handle_arrow_up, handle_tab};

/// Create keydown handler for command execution and navigation
pub fn create_keydown_handler(
    input_value: UseStateHandle<String>,
    cursor_position: UseStateHandle<usize>,
    history: UseStateHandle<Vec<HistoryEntry>>,
    command_history: UseStateHandle<Vec<String>>,
    history_index: UseStateHandle<Option<usize>>,
    executor: UseStateHandle<CommandExecutor>,
    container_ref: NodeRef,
    app_config: UseStateHandle<AppConfigService>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
        "Enter" => {
            e.prevent_default();
            if !input_value.trim().is_empty() {
                handle_enter_key(
                    &input_value,
                    &cursor_position,
                    &history,
                    &command_history,
                    &history_index,
                    &executor,
                    &container_ref,
                    &app_config,
                );
            }
        }
        "ArrowUp" => {
            e.prevent_default();
            handle_arrow_up(
                &command_history,
                &history_index,
                &input_value,
                &cursor_position,
            );
        }
        "ArrowDown" => {
            e.prevent_default();
            handle_arrow_down(
                &command_history,
                &history_index,
                &input_value,
                &cursor_position,
            );
        }
        "Tab" => {
            e.prevent_default();
            handle_tab(&executor, &input_value, &cursor_position);
        }
        _ => {}
    })
}

fn handle_enter_key(
    input_value: &UseStateHandle<String>,
    cursor_position: &UseStateHandle<usize>,
    history: &UseStateHandle<Vec<HistoryEntry>>,
    command_history: &UseStateHandle<Vec<String>>,
    history_index: &UseStateHandle<Option<usize>>,
    executor: &UseStateHandle<CommandExecutor>,
    container_ref: &NodeRef,
    app_config: &UseStateHandle<AppConfigService>,
) {
    let command = (**input_value).clone();

    // Add to command history
    let mut new_command_history = (**command_history).clone();
    if !new_command_history
        .last()
        .map_or(false, |last| last == &command)
    {
        new_command_history.push(command.clone());
    }
    command_history.set(new_command_history);
    history_index.set(None);

    // Execute command
    execute_command(&command, history, executor, app_config);

    // Clear input and scroll
    input_value.set(String::new());
    cursor_position.set(0);

    if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
        let _ = container.set_scroll_top(container.scroll_height());
    }
}

fn execute_command(
    command: &str,
    history: &UseStateHandle<Vec<HistoryEntry>>,
    executor: &UseStateHandle<CommandExecutor>,
    app_config: &UseStateHandle<AppConfigService>,
) {
    let history_clone_for_clear = history.clone();
    let executor_clone_for_execute = executor.clone();
    let app_config_clone_for_theme = app_config.clone();

    let context = TerminalContext {
        app_config: (**app_config).clone(),
        clear_screen: std::rc::Rc::new(move || {
            let welcome_history = vec![create_welcome_entry()];
            history_clone_for_clear.set(welcome_history);
        }),
        command_executor: executor,
        execute: std::rc::Rc::new(move |command_str: &str| {
            let minimal_context = TerminalContext {
                clear_screen: std::rc::Rc::new(|| {}),
                command_executor: &executor_clone_for_execute,
                execute: std::rc::Rc::new(|_| {
                    CommandResult::Error("Nested execute not supported".to_string())
                }),
                app_config: AppConfigService::new(),
                set_theme: None,
            };
            executor_clone_for_execute.execute(command_str, &minimal_context)
        }),
        set_theme: Some(std::rc::Rc::new(move |theme: &str| {
            let mut config = (*app_config_clone_for_theme).clone();
            if config.set_theme(theme) {
                app_config_clone_for_theme.set(config);
                true
            } else {
                false
            }
        })),
    };

    let result = executor.execute(command, &context);
    handle_command_result(result, command.to_string(), history);
}

/// Handle command execution result
pub fn handle_command_result(
    result: CommandResult,
    command: String,
    history: &UseStateHandle<Vec<HistoryEntry>>,
) {
    match result {
        CommandResult::Success(output) => {
            if command.trim() != "clear" {
                let mut current_history = (**history).clone();
                current_history.push(create_command_entry(command, output, false));
                history.set(current_history);
            }
        }
        CommandResult::Error(error) => {
            let mut current_history = (**history).clone();
            current_history.push(create_command_entry(
                command,
                format!("Error: {}", error),
                true,
            ));
            history.set(current_history);
        }
        CommandResult::Html(html_content) => {
            let mut current_history = (**history).clone();
            current_history.push(create_html_entry(command, html_content));
            history.set(current_history);
        }
        CommandResult::Async(future) => {
            let mut current_history = (**history).clone();
            current_history.push(create_command_entry(
                command.clone(),
                "Loading...".to_string(),
                false,
            ));
            let loading_index = current_history.len() - 1;
            history.set(current_history);

            let history_clone = history.clone();
            let command_clone = command.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let async_result = future.await;
                let mut current_history = (*history_clone).clone();

                // Remove loading entry
                if current_history.len() > loading_index {
                    if let Some(entry) = current_history.get(loading_index) {
                        if entry.command_text == command_clone
                            && entry.output.contains("Loading...")
                        {
                            current_history.remove(loading_index);
                        }
                    }
                }

                // Add final result
                match async_result {
                    CommandResult::Success(output) => {
                        current_history.push(create_command_entry(command_clone, output, false));
                    }
                    CommandResult::Error(error) => {
                        current_history.push(create_command_entry(
                            command_clone,
                            format!("Error: {}", error),
                            true,
                        ));
                    }
                    CommandResult::Html(html_content) => {
                        current_history.push(create_html_entry(command_clone, html_content));
                    }
                    CommandResult::Async(_) => {
                        current_history.push(create_command_entry(
                            command_clone,
                            "Error: Nested async operations not supported".to_string(),
                            true,
                        ));
                    }
                }
                history_clone.set(current_history);
            });
        }
    }
}