// Terminal Event Handlers
// Pure functions for handling terminal events

use crate::commands::{CommandExecutor, CommandResult, TerminalContext};
use crate::components::history::{
    create_command_entry, create_html_entry, create_welcome_entry, HistoryEntry,
};
use crate::utils::AppConfigService;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Create input change handler
pub fn create_input_handler(
    input_value: UseStateHandle<String>,
    cursor_position: UseStateHandle<usize>,
    set_trailing: std::rc::Rc<dyn Fn(&str)>,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let new_value = input.value();
        let old_pos = *cursor_position;
        let new_pos = input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

        if new_pos > old_pos {
            set_trailing("cursor-trailing-left");
        } else if new_pos < old_pos {
            set_trailing("cursor-trailing-right");
        }

        input_value.set(new_value);
        cursor_position.set(new_pos);
    })
}

/// Create focus handler
pub fn create_focus_handler(cursor_position: UseStateHandle<usize>) -> Callback<FocusEvent> {
    Callback::from(move |e: FocusEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        cursor_position.set(input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
    })
}

/// Create click handler
pub fn create_click_handler(cursor_position: UseStateHandle<usize>) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        cursor_position.set(input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
    })
}

/// Create terminal area click handler
pub fn create_terminal_click_handler(input_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let _ = input.focus();
        }
    })
}

/// Create keyup handler
pub fn create_keyup_handler(
    cursor_position: UseStateHandle<usize>,
    trailing_class: UseStateHandle<String>,
    trailing_timeout: UseStateHandle<Option<gloo::timers::callback::Timeout>>,
    set_trailing: std::rc::Rc<dyn Fn(&str)>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let old_pos = *cursor_position;
        let new_pos = input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

        match e.key().as_str() {
            "ArrowLeft" | "ArrowRight" => {
                if new_pos > old_pos {
                    set_trailing("cursor-trailing-left");
                } else if new_pos < old_pos {
                    set_trailing("cursor-trailing-right");
                }
            }
            _ => {
                trailing_class.set(String::new());
                trailing_timeout.set(None);
            }
        }

        cursor_position.set(new_pos);
    })
}

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
    let history_clone_for_html = history.clone();
    let command_clone_for_html = command.to_string();
    let executor_clone_for_execute = executor.clone();
    let app_config_clone_for_theme = app_config.clone();

    let context = TerminalContext {
        app_config: (**app_config).clone(),
        clear_screen: std::rc::Rc::new(move || {
            let welcome_history = vec![create_welcome_entry()];
            history_clone_for_clear.set(welcome_history);
        }),
        output_html: std::rc::Rc::new(move |html_content: String| {
            let mut current_history = (*history_clone_for_html).clone();
            current_history.push(create_html_entry(
                command_clone_for_html.clone(),
                html_content,
            ));
            history_clone_for_html.set(current_history);
        }),
        command_executor: executor,
        execute: std::rc::Rc::new(move |command_str: &str| {
            let minimal_context = TerminalContext {
                clear_screen: std::rc::Rc::new(|| {}),
                output_html: std::rc::Rc::new(|_| {}),
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

fn handle_command_result(
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

fn handle_arrow_up(
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

fn handle_arrow_down(
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

fn handle_tab(
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
