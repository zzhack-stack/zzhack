// Main Application Component
// This file contains the root App component that serves as the entry point
// for the terminal emulator application

use crate::components::Terminal;
use crate::utils::use_app_config;
use yew::prelude::*;
use web_sys::window;

/// Main application component that renders the terminal emulator
///
/// This is the root component of the application that contains the terminal
/// interface. It serves as a simple wrapper around the Terminal component.
#[function_component(App)]
pub fn app() -> Html {
    let app_config = use_app_config();
    let is_center_layout = app_config.config.layout.align == "center";
    
    // Apply theme to body element on component mount
    let theme_class = app_config.get_theme_class();
    {
        let theme_class = theme_class.to_string();
        use_effect_with(theme_class.clone(), move |_| {
            if let Some(window) = window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        let _ = body.set_class_name(&theme_class);
                    }
                }
            }
            || {}
        });
    }
    
    let container_class = if is_center_layout {
        format!("w-full h-screen bg-terminal-bg flex justify-center {}", theme_class)
    } else {
        format!("w-full h-screen bg-terminal-bg {}", theme_class)
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
