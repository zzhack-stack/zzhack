// Main Application Component
// This file contains the root App component that serves as the entry point
// for the terminal emulator application

use crate::components::Terminal;
use crate::utils::use_app_config;
use yew::prelude::*;

/// Main application component that renders the terminal emulator
///
/// This is the root component of the application that contains the terminal
/// interface. It serves as a simple wrapper around the Terminal component.
#[function_component(App)]
pub fn app() -> Html {
    let app_config = use_app_config();
    let is_center_layout = app_config.config.layout.align == "center";
    
    let container_class = if is_center_layout {
        "w-full h-screen bg-terminal-bg flex justify-center"
    } else {
        "w-full h-screen bg-terminal-bg"
    };
    
    let terminal_class = if is_center_layout {
        "w-full max-w-[65ch]"
    } else {
        "w-full"
    };

    html! {
        <div class={container_class}>
            <div class={terminal_class}>
                <Terminal />
            </div>
        </div>
    }
}
