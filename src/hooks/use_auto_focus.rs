use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Auto-focus the input field on component mount
#[hook]
pub fn use_auto_focus(input_ref: NodeRef) {
    use_effect_with((), move |_| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let _ = input.focus();
        }
        || {}
    });
}