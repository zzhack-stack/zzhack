// Main Terminal Component Implementation
// This file contains the main terminal UI component that handles user input,
// command execution, and output display in a browser-based terminal emulator

use crate::commands::{CommandExecutor, CommandResult, TerminalContext};
use crate::components::history::{
    create_command_entry, create_html_entry, create_welcome_entry, HistoryItem,
};
use crate::components::syntax::{parse_syntax_segments, render_syntax_segments};
use gloo::timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;


/// Main terminal component that manages the terminal interface
#[function_component(Terminal)]
pub fn terminal() -> Html {
    // State management using hooks
    let input_value = use_state(|| String::new());
    let cursor_position = use_state(|| 0usize); // Track cursor position in input
    let trailing_class = use_state(|| String::new()); // Track trailing direction
    let trailing_timeout = use_state(|| None::<Timeout>); // Track current timeout
    let history = use_state(|| vec![create_welcome_entry()]);
    let command_history = use_state(|| Vec::<String>::new()); // Store command history
    let history_index = use_state(|| None::<usize>); // Current position in history
    let input_ref = use_node_ref();
    let container_ref = use_node_ref();
    let executor = use_state(|| CommandExecutor::new());

    // Focus the input field on component mount and whenever needed
    {
        let input_ref = input_ref.clone();
        use_effect_with((), move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
            // Return a cleanup function (though we don't need it here)
            || {}
        });
    }

    // Note: Removed automatic cursor sync effect to prevent conflicts with native behavior

    // Handle terminal area clicks to focus input
    let on_terminal_click = {
        let input_ref = input_ref.clone();
        Callback::from(move |_e: MouseEvent| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                let _ = input.focus();
            }
        })
    };

    // Helper function to set trailing and clear after delay
    let set_trailing_with_clear = {
        let trailing_class = trailing_class.clone();
        let trailing_timeout = trailing_timeout.clone();

        move |direction: &str| {
            // Cancel existing timeout by replacing it with a new one
            // The previous timeout will be automatically dropped and cleaned up

            // Set new trailing direction
            trailing_class.set(direction.to_string());

            // Set new timeout to clear trailing
            let trailing_class_clear = trailing_class.clone();
            let new_timeout = Timeout::new(80, move || {
                trailing_class_clear.set(String::new());
            });
            trailing_timeout.set(Some(new_timeout));
        }
    };

    // Handle input changes
    let on_input = {
        let input_value = input_value.clone();
        let cursor_position = cursor_position.clone();
        let set_trailing = set_trailing_with_clear.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            let old_pos = *cursor_position;
            let new_pos = input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

            // Detect movement direction and set trailing
            if new_pos > old_pos {
                // Moving right, trail to the left
                set_trailing("cursor-trailing-left");
            } else if new_pos < old_pos {
                // Moving left, trail to the right
                set_trailing("cursor-trailing-right");
            }

            input_value.set(new_value.clone());
            cursor_position.set(new_pos);
        })
    };

    // Handle clicks and selection changes to keep cursor position in sync
    let on_input_focus = {
        let cursor_position = cursor_position.clone();
        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            cursor_position.set(input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
        })
    };

    let on_input_click = {
        let cursor_position = cursor_position.clone();
        Callback::from(move |e: MouseEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            cursor_position.set(input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
        })
    };

    // Handle key up events to sync cursor position and detect arrow key movement
    let on_input_keyup = {
        let cursor_position = cursor_position.clone();
        let set_trailing = set_trailing_with_clear.clone();
        let trailing_class = trailing_class.clone();

        Callback::from(move |e: KeyboardEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let old_pos = *cursor_position;
            let new_pos = input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

            // Detect movement for arrow keys and set trailing
            match e.key().as_str() {
                "ArrowLeft" | "ArrowRight" => {
                    if new_pos > old_pos {
                        set_trailing("cursor-trailing-left");
                    } else if new_pos < old_pos {
                        set_trailing("cursor-trailing-right");
                    }
                }
                _ => {
                    // For other keys, clear trailing immediately
                    trailing_class.set(String::new());
                    // Clear the timeout state
                    trailing_timeout.set(None);
                }
            }

            cursor_position.set(new_pos);
        })
    };

    // Handle command execution and history navigation
    let on_keydown = {
        let input_value = input_value.clone();
        let cursor_position = cursor_position.clone();
        let history = history.clone();
        let command_history = command_history.clone();
        let history_index = history_index.clone();
        let executor = executor.clone();
        let container_ref = container_ref.clone();

        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Enter" => {
                    e.prevent_default(); // Prevent form submission behavior

                    if !input_value.trim().is_empty() {
                        let command = (*input_value).clone();

                        // Add command to command history if it's not empty and different from last
                        let mut new_command_history = (*command_history).clone();
                        if !new_command_history
                            .last()
                            .map_or(false, |last| last == &command)
                        {
                            new_command_history.push(command.clone());
                        }
                        command_history.set(new_command_history);

                        // Reset history index
                        history_index.set(None);

                        // Create terminal context with utility functions
                        let history_clone_for_clear = history.clone();
                        let history_clone_for_html = history.clone();
                        let command_clone_for_html = command.clone();
                        
                        let context = TerminalContext {
                            clear_screen: std::rc::Rc::new(move || {
                                let welcome_history = vec![create_welcome_entry()];
                                history_clone_for_clear.set(welcome_history);
                            }),
                            output_html: std::rc::Rc::new(move |html_content: String| {
                                let mut current_history = (*history_clone_for_html).clone();
                                current_history.push(create_html_entry(command_clone_for_html.clone(), html_content));
                                history_clone_for_html.set(current_history);
                            }),
                        };

                        // Execute the command using the command executor
                        let result = executor.execute(&command, &context);

                        // Handle the result based on its type
                        match result {
                            CommandResult::Success(output) => {
                                let mut current_history = (*history).clone();
                                if !output.is_empty() {
                                    current_history.push(create_command_entry(command.clone(), output, false));
                                } else {
                                    current_history.push(create_command_entry(command.clone(), String::new(), false));
                                }
                                history.set(current_history);
                            }
                            CommandResult::Error(error) => {
                                let mut current_history = (*history).clone();
                                current_history.push(create_command_entry(command.clone(), format!("Error: {}", error), true));
                                history.set(current_history);
                            }
                            CommandResult::Html(html_content) => {
                                let mut current_history = (*history).clone();
                                current_history.push(create_html_entry(command.clone(), html_content));
                                history.set(current_history);
                            }
                            CommandResult::Async(future) => {
                                // Add command to history immediately with loading message
                                let mut current_history = (*history).clone();
                                current_history.push(create_command_entry(command.clone(), "Loading...".to_string(), false));
                                history.set(current_history);
                                
                                // Spawn the async task
                                let history_clone = history.clone();
                                let command_clone = command.clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    let async_result = future.await;
                                    let mut current_history = (*history_clone).clone();
                                    // Remove the "Loading..." entry
                                    current_history.pop();
                                    
                                    match async_result {
                                        CommandResult::Success(output) => {
                                            current_history.push(create_command_entry(command_clone, output, false));
                                        }
                                        CommandResult::Error(error) => {
                                            current_history.push(create_command_entry(command_clone, format!("Error: {}", error), true));
                                        }
                                        CommandResult::Html(html_content) => {
                                            current_history.push(create_html_entry(command_clone, html_content));
                                        }
                                        CommandResult::Async(_) => {
                                            // Nested async not supported, treat as error
                                            current_history.push(create_command_entry(command_clone, "Error: Nested async operations not supported".to_string(), true));
                                        }
                                    }
                                    history_clone.set(current_history);
                                });
                            }
                        }


                        // Clear the input field
                        input_value.set(String::new());
                        cursor_position.set(0);

                        // Scroll to bottom after command execution
                        if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
                            let _ = container.set_scroll_top(container.scroll_height());
                        }
                    }
                }
                "ArrowLeft" | "ArrowRight" => {
                    // Let browser handle native cursor movement naturally
                    // Position will be synced through input events
                }
                "ArrowUp" => {
                    e.prevent_default();
                    let cmd_history = &*command_history;
                    if !cmd_history.is_empty() {
                        let new_index = match *history_index {
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
                "ArrowDown" => {
                    e.prevent_default();
                    let cmd_history = &*command_history;
                    if !cmd_history.is_empty() {
                        match *history_index {
                            None => {} // Do nothing if not navigating history
                            Some(idx) => {
                                if idx < cmd_history.len() - 1 {
                                    let new_index = idx + 1;
                                    history_index.set(Some(new_index));
                                    input_value.set(cmd_history[new_index].clone());
                                    cursor_position.set(cmd_history[new_index].len());
                                } else {
                                    // Go beyond last command - clear input
                                    history_index.set(None);
                                    input_value.set(String::new());
                                    cursor_position.set(0);
                                }
                            }
                        }
                    }
                }
                "Tab" => {
                    e.prevent_default();

                    // Get completion suggestions from local executor
                    let (suggestions, prefix) =
                        executor.get_completion_suggestions(&*input_value, *cursor_position);

                    if !suggestions.is_empty() {
                        // Take the first suggestion
                        let suggestion = &suggestions[0];

                        // Calculate the replacement
                        let current_input = (*input_value).clone();
                        let cursor_pos = *cursor_position;

                        // Find where the prefix starts
                        let prefix_start = if cursor_pos >= prefix.len() {
                            cursor_pos - prefix.len()
                        } else {
                            0
                        };

                        // Build the new input string
                        let mut new_input = String::new();
                        new_input.push_str(&current_input[..prefix_start]);
                        new_input.push_str(suggestion);
                        new_input.push_str(&current_input[cursor_pos..]);

                        // Calculate new cursor position
                        let new_cursor_pos = prefix_start + suggestion.len();

                        // Update the input and cursor position
                        input_value.set(new_input);
                        cursor_position.set(new_cursor_pos);
                    }
                }
                _ => {} // Ignore other keys
            }
        })
    };

    html! {
        <div class="w-full h-full bg-terminal-bg text-terminal-text font-mono flex flex-col">
            // Terminal header
            <div class="bg-terminal-header px-4 py-2 border-b border-terminal-border">
                <span class="text-sm font-bold">{"Terminal Emulator"}</span>
            </div>

            // Terminal body - all content in one scrollable container
            <div ref={container_ref} class="flex-1 p-4 overflow-y-auto terminal-scrollbar cursor-text" onclick={on_terminal_click}>
                // Command history
                {for history.iter().map(|entry| {
                    let valid_commands = (*executor).get_command_names();
                    html! {
                        <HistoryItem entry={entry.clone()} valid_commands={valid_commands} />
                    }
                })}

                // Input line with prompt and syntax highlighted content - directly after history
                <div class="flex items-start">
                    <span class="text-green-500 mr-2 mt-0.5 text-sm font-mono">{"$ "}</span>
                    <div class="flex-1 relative">
                        // Hidden input for actual typing
                        <input
                            ref={input_ref}
                            type="text"
                            class="absolute inset-0 w-full bg-transparent border-none text-transparent text-sm outline-none py-0.5 font-mono"
                            value={(*input_value).clone()}
                            oninput={on_input}
                            onkeydown={on_keydown}
                            onkeyup={on_input_keyup}
                            onfocus={on_input_focus}
                            onclick={on_input_click}
                            autofocus=true
                            style="z-index: 4; caret-color: transparent;"
                        />
                        // Visible syntax highlighted overlay
                        <div class="absolute inset-0 text-sm py-0.5 font-mono pointer-events-none" style="z-index: 2;">
                            {if !input_value.is_empty() {
                                let valid_commands = (*executor).get_command_names();
                                let segments = parse_syntax_segments(&*input_value, &valid_commands);
                                render_syntax_segments(&segments)
                            } else {
                                html! {}
                            }}
                        </div>


                        // Custom cursor as a separate div with dynamic trailing effect
                        <div
                            class={format!("absolute bg-green-500 cursor-blink pointer-events-none {}", *trailing_class)}
                            style={format!("left: {}px; top: 0.125rem; width: 8px; height: 18px; z-index: 3;",
                                (*cursor_position as f32 * 8.4))}
                        ></div>
                    </div>
                </div>
            </div>
        </div>
    }
}
