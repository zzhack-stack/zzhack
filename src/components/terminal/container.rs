// Terminal Container Component
// Main container that manages all terminal state and coordinates child components

use crate::commands::CommandExecutor;
use crate::components::history::create_welcome_entry;
use super::content::TerminalContent;
use super::handlers::*;
use super::header::TerminalHeader;
use super::hooks::{use_auto_focus, use_auto_navigation, use_trailing_effect};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalProps {
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component(Terminal)]
pub fn terminal(props: &TerminalProps) -> Html {
    // State management
    let input_value = use_state(|| String::new());
    let cursor_position = use_state(|| 0usize);
    let history = use_state(|| vec![create_welcome_entry()]);
    let command_history = use_state(|| Vec::<String>::new());
    let history_index = use_state(|| None::<usize>);
    let input_ref = use_node_ref();
    let container_ref = use_node_ref();
    let executor = use_state(|| CommandExecutor::new());

    // Custom hooks
    use_auto_focus(input_ref.clone());
    use_auto_navigation(history.clone(), executor.clone());
    let (trailing_class, trailing_timeout, set_trailing) = use_trailing_effect();

    // Event handlers
    let on_terminal_click = create_terminal_click_handler(input_ref.clone());
    
    let on_input = create_input_handler(
        input_value.clone(),
        cursor_position.clone(),
        set_trailing.clone(),
    );
    
    let on_focus = create_focus_handler(cursor_position.clone());
    let on_click = create_click_handler(cursor_position.clone());
    
    let on_keyup = create_keyup_handler(
        cursor_position.clone(),
        trailing_class.clone(),
        trailing_timeout.clone(),
        set_trailing.clone(),
    );
    
    let on_keydown = create_keydown_handler(
        input_value.clone(),
        cursor_position.clone(),
        history.clone(),
        command_history.clone(),
        history_index.clone(),
        executor.clone(),
        container_ref.clone(),
    );

    html! {
        <div class="w-full h-full bg-terminal-bg text-terminal-text font-mono flex flex-col">
            <TerminalHeader title={props.title.clone()} />
            
            <TerminalContent
                container_ref={container_ref}
                history={(*history).clone()}
                executor={(*executor).clone()}
                input_value={(*input_value).clone()}
                cursor_position={*cursor_position}
                trailing_class={(*trailing_class).clone()}
                input_ref={input_ref}
                on_terminal_click={on_terminal_click}
                on_input={on_input}
                on_keydown={on_keydown}
                on_keyup={on_keyup}
                on_focus={on_focus}
                on_click={on_click}
            />
        </div>
    }
}