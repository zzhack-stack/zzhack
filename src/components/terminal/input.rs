// Terminal Input Component
// Handles all input-related functionality including cursor, syntax highlighting, and events

use crate::commands::CommandExecutor;
use crate::components::syntax::{parse_syntax_segments, render_syntax_segments};
use crate::utils::use_app_config;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalInputProps {
    pub input_value: String,
    pub cursor_position: usize,
    pub trailing_class: String,
    pub executor: CommandExecutor,
    pub input_ref: NodeRef,
    pub on_input: Callback<InputEvent>,
    pub on_keydown: Callback<KeyboardEvent>,
    pub on_keyup: Callback<KeyboardEvent>,
    pub on_focus: Callback<FocusEvent>,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(TerminalInput)]
pub fn terminal_input(props: &TerminalInputProps) -> Html {
    html! {
        <div class="flex items-start">
            <TerminalPrompt />
            <div class="flex-1 relative">
                <HiddenInput
                    input_ref={props.input_ref.clone()}
                    value={props.input_value.clone()}
                    on_input={props.on_input.clone()}
                    on_keydown={props.on_keydown.clone()}
                    on_keyup={props.on_keyup.clone()}
                    on_focus={props.on_focus.clone()}
                    on_click={props.on_click.clone()}
                />
                <SyntaxHighlight
                    input_value={props.input_value.clone()}
                    executor={props.executor.clone()}
                />
                <TerminalCursor
                    position={props.cursor_position}
                    trailing_class={props.trailing_class.clone()}
                />
            </div>
        </div>
    }
}

#[function_component(TerminalPrompt)]
fn terminal_prompt() -> Html {
    let app_config = use_app_config();

    html! {
        <span class="mr-2 mt-0.5 text-sm font-mono font-bold" style={format!("color: {}", app_config.config.terminal.color)}>
            {format!("{} ", app_config.config.terminal.prompt)}
        </span>
    }
}

#[derive(Properties, PartialEq)]
struct HiddenInputProps {
    pub input_ref: NodeRef,
    pub value: String,
    pub on_input: Callback<InputEvent>,
    pub on_keydown: Callback<KeyboardEvent>,
    pub on_keyup: Callback<KeyboardEvent>,
    pub on_focus: Callback<FocusEvent>,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(HiddenInput)]
fn hidden_input(props: &HiddenInputProps) -> Html {
    html! {
        <input
            ref={props.input_ref.clone()}
            type="text"
            class="absolute inset-0 w-full bg-transparent border-none text-transparent text-sm outline-none py-0.5 font-mono"
            value={props.value.clone()}
            oninput={props.on_input.clone()}
            onkeydown={props.on_keydown.clone()}
            onkeyup={props.on_keyup.clone()}
            onfocus={props.on_focus.clone()}
            onclick={props.on_click.clone()}
            autofocus=true
            style="z-index: 4; caret-color: transparent;"
        />
    }
}

#[derive(Properties, PartialEq)]
struct SyntaxHighlightProps {
    pub input_value: String,
    pub executor: CommandExecutor,
}

#[function_component(SyntaxHighlight)]
fn syntax_highlight(props: &SyntaxHighlightProps) -> Html {
    html! {
        <div class="absolute inset-0 text-sm py-0.5 font-mono pointer-events-none" style="z-index: 2;">
            {if !props.input_value.is_empty() {
                let valid_commands = props.executor.get_command_names();
                let segments = parse_syntax_segments(&props.input_value, &valid_commands);
                render_syntax_segments(&segments)
            } else {
                html! {}
            }}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TerminalCursorProps {
    pub position: usize,
    pub trailing_class: String,
}

#[function_component(TerminalCursor)]
fn terminal_cursor(props: &TerminalCursorProps) -> Html {
    let app_config = use_app_config();

    html! {
        <div
            class={format!("absolute cursor-blink pointer-events-none {}", props.trailing_class)}
            style={format!("left: {}px; top: 0.125rem; width: 8px; height: 18px; z-index: 3; background-color: {};",
                (props.position as f32 * 8.4), app_config.config.terminal.color)}
        ></div>
    }
}
