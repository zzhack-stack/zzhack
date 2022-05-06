use crate::components::common::layout::BaseLayout;
use crate::global::theme_context::ThemeProvider;
use crate::routes::RouteOutlet;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeProvider>
            <BaseLayout>
                <RouteOutlet />
            </BaseLayout>
        </ThemeProvider>
    }
}
