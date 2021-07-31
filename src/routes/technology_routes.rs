use crate::pages::{
    home::Home,
    technology::{Collection, OpenSource},
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum TechnologyRoutes {
    #[to = "/technology/collection"]
    Collection,
    #[to = "/technology/vector"]
    OpenSource,
}

pub type TechnologyRouterAnchor = RouterAnchor<TechnologyRoutes>;

pub fn switch(routes: TechnologyRoutes) -> Html {
    match routes {
        TechnologyRoutes::Collection => {
            html! { <Collection />}
        }
        TechnologyRoutes::OpenSource => {
            html! { <OpenSource />}
        }
        _ => html! {<Home />},
    }
}

pub type TechnologyRouter = Router<TechnologyRoutes>;
