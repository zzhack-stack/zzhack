use crate::pages::{
    home::Home,
    technology::{Articles, OpenSource},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum TechnologyRoutes {
    #[to = "/technology/articles"]
    Articles,
    #[to = "/technology/open_source"]
    OpenSource,
}

pub type TechnologyRouterAnchor = RouterAnchor<TechnologyRoutes>;

pub fn switch(routes: TechnologyRoutes) -> Html {
    match routes {
        TechnologyRoutes::Articles => {
            html! { <Articles />}
        }
        TechnologyRoutes::OpenSource => {
            html! { <OpenSource />}
        }
        _ => html! {<Home />},
    }
}

pub type TechnologyRouter = Router<TechnologyRoutes>;
