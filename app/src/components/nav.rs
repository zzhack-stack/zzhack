use shared::site_config::NavConfig;
use yew::prelude::*;

use crate::{components::theme_img::ThemeImg, icons::GitHubIcon};

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub nav_configs: Vec<NavConfig>,
}

#[function_component]
pub fn Nav() -> Html {
    let site_config = site_config::get_site_config();
    let rendered_nav_items = site_config.nav.iter().map(|config| {
        html! {
            <a class="font-semibold ml-5 text-sm" href={config.url.clone()}>{&config.text}</a>
        }
    });

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <div class="flex items-center justify-between	">
                <div>
                    <ThemeImg class="h-8" src="/assets/zzhack-logo.png" alt="logo" />
                </div>
                <div class="navbar-start flex items-center">
                    {for rendered_nav_items}
                    <div class="w-px h-3 bg-black mx-5" />
                    <GitHubIcon color="" height="17" width="16" />
                </div>
            </div>
        </nav>
    }
}
