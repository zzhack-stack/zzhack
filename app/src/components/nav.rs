use site_config::NavConfig;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub nav_configs: Vec<NavConfig>,
}

#[function_component]
pub fn Nav(props: &NavProps) -> Html {
    let rendered_nav_items = props.nav_configs.iter().map(|config| {
        html! {
            <a class="navbar-item" href={config.url.clone()}>{&config.text}</a>
        }
    });

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="navbar-menu">
                <div class="navbar-start">
                    {for rendered_nav_items}
                </div>
            </div>
        </nav>
    }
}
