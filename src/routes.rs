use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::Home,
    about_me::AboutMe,
    not_found::NotFound,
};

#[derive(Debug, Switch, Clone)]
pub enum Routes {
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
}

pub type AppRouterAnchor = RouterAnchor<Routes>;

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Home => {
            html! { <Home />}
        },
        Routes::AboutMe => {
            html! { <AboutMe />}
        },
        Routes::NotFound => {
            html! {<NotFound />}
        },
        _ =>  html! {<Home />}
    }
}