// Terminal Header Component
// Simple, reusable header component following React component philosophy

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalHeaderProps {
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component(TerminalHeader)]
pub fn terminal_header(props: &TerminalHeaderProps) -> Html {
    let title = props.title.as_deref().unwrap_or("Terminal Emulator");
    
    html! {
        <div class="bg-terminal-header px-4 py-2 border-b border-terminal-border">
            <span class="text-sm font-bold">{title}</span>
        </div>
    }
}