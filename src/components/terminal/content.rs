// Terminal Content Component
// Container component that manages the scrollable content area

use crate::commands::CommandExecutor;
use crate::components::history::HistoryEntry;
use super::history::TerminalHistory;
use super::input::TerminalInput;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalContentProps {
    pub container_ref: NodeRef,
    pub history: Vec<HistoryEntry>,
    pub executor: CommandExecutor,
    pub input_value: String,
    pub cursor_position: usize,
    pub trailing_class: String,
    pub input_ref: NodeRef,
    pub on_terminal_click: Callback<MouseEvent>,
    pub on_input: Callback<InputEvent>,
    pub on_keydown: Callback<KeyboardEvent>,
    pub on_keyup: Callback<KeyboardEvent>,
    pub on_focus: Callback<FocusEvent>,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(TerminalContent)]
pub fn terminal_content(props: &TerminalContentProps) -> Html {
    html! {
        <div 
            ref={props.container_ref.clone()} 
            class="flex-1 p-4 overflow-y-auto terminal-scrollbar cursor-text" 
            onclick={props.on_terminal_click.clone()}
        >
            <TerminalHistory 
                history={props.history.clone()} 
                executor={props.executor.clone()}
            />
            
            <TerminalInput
                input_value={props.input_value.clone()}
                cursor_position={props.cursor_position}
                trailing_class={props.trailing_class.clone()}
                executor={props.executor.clone()}
                input_ref={props.input_ref.clone()}
                on_input={props.on_input.clone()}
                on_keydown={props.on_keydown.clone()}
                on_keyup={props.on_keyup.clone()}
                on_focus={props.on_focus.clone()}
                on_click={props.on_click.clone()}
            />
        </div>
    }
}