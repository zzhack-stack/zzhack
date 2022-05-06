use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{about::About, home::Home, not_found::NotFound, projects::Projects};

#[derive(Clone, Routable, PartialEq, Debug)]
enum RootRoutes {
    #[at("/home")]
    Home,
    #[at("/")]
    Root,
    #[at("/projects")]
    Projects,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &RootRoutes) -> Html {
    match routes {
        RootRoutes::Home | RootRoutes::Root => html! { <Home /> },
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::About => html! { <About /> },
        RootRoutes::NotFound => html! { <NotFound />},
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
            <Switch<RootRoutes> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
