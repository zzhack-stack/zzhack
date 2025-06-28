// Main entry point for the terminal emulator WASM application
// This file initializes the Yew application and mounts it to the DOM

mod app;
mod commands;
mod components;
mod filesystem;
mod utils;

use app::App;

/// Main function that initializes and starts the Yew application
/// This is called when the WASM module is loaded in the browser
fn main() {
    // Initialize the Yew renderer with our main App component
    // and render it to the DOM element with id "app"
    yew::Renderer::<App>::new().render();
}
