// Main Application Component
// This file contains the root App component that serves as the entry point
// for the terminal emulator application

use crate::components::Terminal;
use crate::hooks::use_app_config;
use web_sys::window;
use yew::prelude::*;

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
        use_effect_with(theme_class.to_string().clone(), move |_| {
            window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .set_class_name(&theme_class);
        });
    }

    let terminal_class = if is_center_layout {
        "w-full max-w-[65ch]"
    } else {
        "w-full"
    };

    html! {
        <div class={vec![format!("w-full h-screen bg-terminal-bg overflow-y-scroll {} {}", theme_class, if is_center_layout {"flex justify-center"} else {""}), ]}>
            <div class={terminal_class}>
                <Terminal />
            </div>
        </div>
    }
}
