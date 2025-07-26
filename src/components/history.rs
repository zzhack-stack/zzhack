// Terminal History Management
// This module handles terminal history entries and their rendering

use crate::components::ansi::{parse_ansi_text, render_ansi_segments};
use crate::components::syntax::render_command_with_syntax;
use crate::utils::use_app_config;
use yew::prelude::*;

/// Represents a single entry in the terminal history
/// Each entry contains the command that was executed and its output
#[derive(Clone, PartialEq)]
pub struct HistoryEntry {
    /// The command that was executed (includes the prompt)
    pub command: String,
    /// The original command text for syntax highlighting
    pub command_text: String,
    /// The output produced by the command
    pub output: String,
    /// Whether this output is an error (affects styling)
    pub is_error: bool,
    /// Whether the output contains HTML that should be rendered
    pub is_html: bool,
}

/// Properties for the HistoryItem component
#[derive(Properties, PartialEq)]
pub struct HistoryItemProps {
    pub entry: HistoryEntry,
    pub valid_commands: Vec<String>,
}

/// Component that renders a single history entry
#[function_component(HistoryItem)]
pub fn history_item(props: &HistoryItemProps) -> Html {
    // Hooks must be at the top level
    let html_ref = use_node_ref();
    let app_config = use_app_config();

    // Set up effect for HTML content
    {
        let html_ref = html_ref.clone();
        let output = props.entry.output.clone();
        let is_html = props.entry.is_html;

        use_effect_with((output.clone(), is_html), move |_| {
            if is_html {
                if let Some(element) = html_ref.cast::<web_sys::HtmlElement>() {
                    element.set_inner_html(&output);
                }
            }
            || {}
        });
    }

    // Parse ANSI sequences in the output
    let segments = parse_ansi_text(&props.entry.output);

    // Render output based on content type
    let output_content = if props.entry.is_html {
        html! {
            <div ref={html_ref} class="markdown-content prose prose-invert max-w-none text-sm" />
        }
    } else if segments.len() > 1
        || segments
            .get(0)
            .map_or(false, |s| s.color.is_some() || s.bold || s.italic)
    {
        // Has ANSI styling - render with parsed segments
        render_ansi_segments(&segments)
    } else {
        // No ANSI styling - use fallback colors
        let output_class = if props.entry.command.is_empty() {
            // Welcome message gets special styling
            "text-terminal-info italic"
        } else if props.entry.is_error {
            "text-terminal-error"
        } else {
            // Default to white for regular output
            "text-white"
        };

        html! {
            <span class={classes!("whitespace-pre-wrap", "text-sm", "font-mono", output_class)}>
                {&props.entry.output}
            </span>
        }
    };

    html! {
        <div class="mb-2">
            // Show command line if it's not empty (skip for welcome message)
            {if !props.entry.command.is_empty() {
                html! {
                    <div class="mb-1 flex items-start">
                        <span class="mr-2 text-sm font-mono" style={format!("color: {}", app_config.config.terminal.color)}>
                            {format!("{} ", app_config.config.terminal.prompt)}
                        </span>
                        <div class="flex-1 text-sm font-mono">
                            {render_command_with_syntax(&props.entry.command_text, &props.valid_commands)}
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            // Show command output with ANSI styling or fallback colors
            <div class="ml-4 text-sm font-mono">
                {output_content}
            </div>
        </div>
    }
}

/// Create initial welcome history entry (empty - no welcome message)
pub fn create_welcome_entry() -> HistoryEntry {
    HistoryEntry {
        command: String::new(),
        command_text: String::new(),
        output: String::new(),
        is_error: false,
        is_html: false,
    }
}

/// Create a new command history entry
pub fn create_command_entry(command_text: String, output: String, is_error: bool) -> HistoryEntry {
    HistoryEntry {
        command: "command".to_string(), // This will be used to check if it's not empty
        command_text,
        output,
        is_error,
        is_html: false,
    }
}

/// Create a new command history entry with HTML content
pub fn create_html_entry(command_text: String, html_output: String) -> HistoryEntry {
    HistoryEntry {
        command: "command".to_string(),
        command_text,
        output: html_output,
        is_error: false,
        is_html: true,
    }
}
