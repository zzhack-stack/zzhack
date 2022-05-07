use crate::routes::RouteOutlet;
use global::theme_context::ThemeProvider;
use ui::common::layout::BaseLayout;
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
