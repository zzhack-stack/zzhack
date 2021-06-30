use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::Home,
    about_me::AboutMe,
    not_found::NotFound,
    technology::{
        Articles,
        OpenSource,
    },
};

#[derive(Debug, Switch, Clone)]
pub enum TechnologyRoutes {
    #[to="/technology/articles"]
    Articles,
    #[to="/technology/open_source"]
    OpenSource,
}

pub type TechnologyRouterAnchor = RouterAnchor<TechnologyRoutes>;

pub fn switch(routes: TechnologyRoutes) -> Html {
    match routes {
        TechnologyRoutes::Articles => {
            html! { <Articles />}
        },
        TechnologyRoutes::OpenSource => {
            html! { <OpenSource />}
        },
        _ =>  html! {<Home />}
    }
}

pub type TechnologyRouter = Router<TechnologyRoutes>;
