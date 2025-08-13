// Terminal Event Handlers Module
// Modular organization of different terminal event handlers

pub mod command_handlers;
pub mod completion_handlers;
pub mod input_handlers;
pub mod navigation_handlers;

// Re-export main handler creation functions
pub use command_handlers::create_keydown_handler;
pub use completion_handlers::handle_tab;
pub use input_handlers::{create_click_handler, create_focus_handler, create_input_handler, create_keyup_handler, create_terminal_click_handler};
pub use navigation_handlers::{handle_arrow_down, handle_arrow_up};