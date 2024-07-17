use std::collections::HashMap;

use crate::components::nav::Nav;

use super::routes::{switch, Routes};
use site_config::Config;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct BrowserAppProps {
    pub config: Config,
}

#[function_component]
pub fn BrowserApp(props: &BrowserAppProps) -> Html {
    html! {
        <BrowserRouter>
            <Nav nav_configs={props.config.nav.clone()} />
            <main class="p-4 h-full w-full">
                <Switch<Routes> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
    pub config: Config,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    let inject_script = format!(
        "window.siteConfig = {}",
        serde_json::to_string(&props.config).unwrap()
    );

    // Sync server route state to browser route state
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Nav nav_configs={props.config.nav.clone()} />
            <main class="p-4 h-full w-full">
                <Switch<Routes> render={switch} />
            </main>
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
            <script>{inject_script}</script>
        </Router>
    }
}
