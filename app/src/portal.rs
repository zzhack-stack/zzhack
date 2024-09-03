use std::collections::HashMap;

use crate::components::nav::Nav;

use super::routes::{switch, Routes};
use shared::site_config::Config;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct BrowserAppProps {
    pub config: Config,
}

#[function_component]
pub fn App() -> Html {
    let cb = Callback::from(|_| {
        let document = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .document_element()
            .unwrap();

        document.set_class_name("dark");
    });

    html! {
        <main class="p-4 h-full w-full flex justify-center bg-white-200 dark:bg-black-900">
            <div class="h-full w-[655px]">
                <Nav />
                <Switch<Routes> render={switch} />
            </div>
            <button onclick={cb}>{"Dark"}</button>
        </main>
    }
}

#[function_component]
pub fn BrowserApp() -> Html {
    html! {
        <BrowserRouter>
            <App />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());

    // Sync server route state to browser route state
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
           <App />
        </Router>
    }
}
