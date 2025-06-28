// Terminal Hooks
// Custom hooks for terminal functionality following React hooks pattern

use crate::commands::{CommandExecutor, CommandResult, TerminalContext};
use crate::components::history::{
    create_command_entry, create_html_entry, create_welcome_entry, HistoryEntry,
};
use crate::utils::config::{get_base_url, start_with_slash};
use gloo::timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Auto-focus the input field on component mount
#[hook]
pub fn use_auto_focus(input_ref: NodeRef) {
    use_effect_with((), move |_| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let _ = input.focus();
        }
        || {}
    });
}

/// Auto-execute navigate command based on current pathname
#[hook]
pub fn use_auto_navigation(
    history: UseStateHandle<Vec<HistoryEntry>>,
    executor: UseStateHandle<CommandExecutor>,
) {
    use_effect_with((), move |_| {
        execute_auto_navigation(history, executor);
        || {}
    });
}

/// Hook for managing trailing cursor effect
#[hook]
pub fn use_trailing_effect() -> (
    UseStateHandle<String>,
    UseStateHandle<Option<Timeout>>,
    std::rc::Rc<dyn Fn(&str)>,
) {
    let trailing_class = use_state(|| String::new());
    let trailing_timeout = use_state(|| None::<Timeout>);

    let set_trailing = {
        let trailing_class = trailing_class.clone();
        let trailing_timeout = trailing_timeout.clone();

        std::rc::Rc::new(move |direction: &str| {
            trailing_class.set(direction.to_string());
            let trailing_class_clear = trailing_class.clone();
            let new_timeout = Timeout::new(80, move || {
                trailing_class_clear.set(String::new());
            });
            trailing_timeout.set(Some(new_timeout));
        }) as std::rc::Rc<dyn Fn(&str)>
    };

    (trailing_class, trailing_timeout, set_trailing)
}

fn execute_auto_navigation(
    history: UseStateHandle<Vec<HistoryEntry>>,
    executor: UseStateHandle<CommandExecutor>,
) {
    if let Some(window) = web_sys::window() {
        let location = window.location();

        if let Ok(pathname) = location.pathname() {
            let baseurl = get_base_url();
            let navigate_path = if pathname == baseurl {
                "/".to_string()
            } else if pathname.starts_with(&format!("{}/", baseurl)) {
                pathname
                    .strip_prefix(&baseurl)
                    .unwrap_or(&pathname)
                    .to_string()
            } else if pathname == "/" {
                "/".to_string()
            } else {
                pathname.trim_start_matches('/').to_string()
            };

            let navigate_command = format!("navigate {}", start_with_slash(&navigate_path));

            execute_auto_command(&navigate_command, history, executor);
        }
    }
}

fn execute_auto_command(
    command: &str,
    history: UseStateHandle<Vec<HistoryEntry>>,
    executor: UseStateHandle<CommandExecutor>,
) {
    let navigate_command_for_html = command.to_string();
    let navigate_command_for_async = command.to_string();

    // Create terminal context for auto-execution
    let history_clone_for_clear = history.clone();
    let history_clone_for_html = history.clone();
    let executor_clone_for_execute = executor.clone();

    let context = TerminalContext {
        clear_screen: std::rc::Rc::new(move || {
            let welcome_history = vec![create_welcome_entry()];
            history_clone_for_clear.set(welcome_history);
        }),
        output_html: std::rc::Rc::new(move |html_content: String| {
            let mut current_history = (*history_clone_for_html).clone();
            current_history.push(create_html_entry(
                navigate_command_for_html.clone(),
                html_content,
            ));
            history_clone_for_html.set(current_history);
        }),
        command_executor: &executor,
        execute: std::rc::Rc::new(move |command_str: &str| {
            let minimal_context = TerminalContext {
                clear_screen: std::rc::Rc::new(|| {}),
                output_html: std::rc::Rc::new(|_| {}),
                command_executor: &executor_clone_for_execute,
                execute: std::rc::Rc::new(|_| {
                    CommandResult::Error("Nested execute not supported".to_string())
                }),
            };
            executor_clone_for_execute.execute(command_str, &minimal_context)
        }),
    };

    // Execute the navigate command
    let result = executor.execute(command, &context);

    // Handle the result and add to history
    handle_auto_navigation_result(
        result,
        command.to_string(),
        navigate_command_for_async,
        &history,
    );
}

fn handle_auto_navigation_result(
    result: CommandResult,
    command: String,
    navigate_command_for_async: String,
    history: &UseStateHandle<Vec<HistoryEntry>>,
) {
    match result {
        CommandResult::Success(output) => {
            if !output.is_empty() {
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
            // Add loading entry
            let mut current_history = (**history).clone();
            current_history.push(create_command_entry(
                command.clone(),
                "Loading...".to_string(),
                false,
            ));
            let loading_index = current_history.len() - 1;
            history.set(current_history);

            // Handle async result
            let history_clone = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let async_result = future.await;
                let mut current_history = (*history_clone).clone();

                // Remove loading entry
                if current_history.len() > loading_index {
                    if let Some(entry) = current_history.get(loading_index) {
                        if entry.command_text == navigate_command_for_async
                            && entry.output.contains("Loading...")
                        {
                            current_history.remove(loading_index);
                        }
                    }
                }

                // Add final result
                match async_result {
                    CommandResult::Success(output) => {
                        current_history.push(create_command_entry(
                            navigate_command_for_async.clone(),
                            output,
                            false,
                        ));
                    }
                    CommandResult::Error(error) => {
                        current_history.push(create_command_entry(
                            navigate_command_for_async.clone(),
                            format!("Error: {}", error),
                            true,
                        ));
                    }
                    CommandResult::Html(html_content) => {
                        current_history
                            .push(create_html_entry(navigate_command_for_async, html_content));
                    }
                    CommandResult::Async(_) => {
                        current_history.push(create_command_entry(
                            navigate_command_for_async,
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
