use crate::routes::RouteOutlet;
use global::theme_context::ThemeProvider;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeProvider>
            <RouteOutlet />
        </ThemeProvider>
    }
}
