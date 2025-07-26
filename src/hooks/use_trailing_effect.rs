use gloo::timers::callback::Timeout;
use yew::prelude::*;

/// Hook for managing trailing cursor effect
#[hook]
pub fn use_trailing_effect() -> (
    UseStateHandle<String>,
    UseStateHandle<Option<Timeout>>,
    std::rc::Rc<dyn Fn(&str)>,
) {
    let trailing_class = use_state(|| String::new());
    let trailing_timeout = use_state(|| None::<Timeout>);

    let set_trailing = {
        let trailing_class = trailing_class.clone();
        let trailing_timeout = trailing_timeout.clone();

        std::rc::Rc::new(move |direction: &str| {
            trailing_class.set(direction.to_string());
            let trailing_class_clear = trailing_class.clone();
            let new_timeout = Timeout::new(80, move || {
                trailing_class_clear.set(String::new());
            });
            trailing_timeout.set(Some(new_timeout));
        }) as std::rc::Rc<dyn Fn(&str)>
    };

    (trailing_class, trailing_timeout, set_trailing)
}