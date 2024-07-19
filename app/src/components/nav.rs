use shared::site_config::{Config, NavConfig};
// use shared::site_config::NavConfig;
use yew::prelude::*;

use crate::utils::site_config::get_config;

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub nav_configs: Vec<NavConfig>,
}

#[function_component]
pub fn Nav() -> Html {
    let site_config = site_config::get_site_config();
    let rendered_nav_items = site_config.nav.iter().map(|config| {
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
