// Input Event Handlers
// Handlers for terminal input interactions (mouse, keyboard, focus)

use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Create input change handler
pub fn create_input_handler(
    input_value: UseStateHandle<String>,
    cursor_position: UseStateHandle<usize>,
    set_trailing: std::rc::Rc<dyn Fn(&str)>,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let new_value = input.value();
        let old_pos = *cursor_position;
        let new_pos = input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

        if new_pos > old_pos {
            set_trailing("cursor-trailing-left");
        } else if new_pos < old_pos {
            set_trailing("cursor-trailing-right");
        }

        input_value.set(new_value);
        cursor_position.set(new_pos);
    })
}

/// Create focus handler
pub fn create_focus_handler(cursor_position: UseStateHandle<usize>) -> Callback<FocusEvent> {
    Callback::from(move |e: FocusEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        cursor_position.set(input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
    })
}

/// Create click handler
pub fn create_click_handler(cursor_position: UseStateHandle<usize>) -> Callback<MouseEvent> {
    Callback::from(move |e: MouseEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        cursor_position.set(input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize);
    })
}

/// Create terminal area click handler
pub fn create_terminal_click_handler(input_ref: NodeRef) -> Callback<MouseEvent> {
    Callback::from(move |_e: MouseEvent| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let _ = input.focus();
        }
    })
}

/// Create keyup handler
pub fn create_keyup_handler(
    cursor_position: UseStateHandle<usize>,
    trailing_class: UseStateHandle<String>,
    trailing_timeout: UseStateHandle<Option<gloo::timers::callback::Timeout>>,
    set_trailing: std::rc::Rc<dyn Fn(&str)>,
) -> Callback<KeyboardEvent> {
    Callback::from(move |e: KeyboardEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let old_pos = *cursor_position;
        let new_pos = input.selection_start().unwrap_or(Some(0)).unwrap_or(0) as usize;

        match e.key().as_str() {
            "ArrowLeft" | "ArrowRight" => {
                if new_pos > old_pos {
                    set_trailing("cursor-trailing-left");
                } else if new_pos < old_pos {
                    set_trailing("cursor-trailing-right");
                }
            }
            _ => {
                trailing_class.set(String::new());
                trailing_timeout.set(None);
            }
        }

        cursor_position.set(new_pos);
    })
}