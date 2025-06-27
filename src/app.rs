// Main Application Component
// This file contains the root App component that serves as the entry point
// for the terminal emulator application

use crate::components::terminal::Terminal;
use yew::prelude::*;

/// Main application component that renders the terminal emulator
///
/// This is the root component of the application that contains the terminal
/// interface. It serves as a simple wrapper around the Terminal component.
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="w-full h-screen bg-terminal-bg">
            // Render the main terminal component
            <Terminal />
        </div>
    }
}
