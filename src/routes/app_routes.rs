use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::Home,
    about_me::AboutMe,
    not_found::NotFound,
    technology::Technology,
};
use crate::console_log;

#[derive(Debug, Switch, Clone)]
pub enum AppRoutes {
    #[to="/about/me"]
    AboutMe,
    #[to="/technology"]
    Technology,
    #[to="/thinking"]
    Thinking,
    #[to="/404"]
    NotFound,
    #[to="/"]
    Home,
    #[to="/technology/articles"]
    Articles,
    #[to="/technology/open_source"]
    OpenSource,
}

pub type AppRouterAnchor = RouterAnchor<AppRoutes>;

pub fn switch(routes: AppRoutes) -> Html {
    match routes {
        AppRoutes::Home => {
            html! { <Home />}
        },
        AppRoutes::AboutMe => {
            html! { <AboutMe />}
        },
        AppRoutes::NotFound => {
            html! {<NotFound />}
        },
        AppRoutes::Technology => {
            html! {<Technology />}
        },
        _ =>  html! {<NotFound />}
    }
}