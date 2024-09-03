use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{dynamic::Dynamic, links::Links, not_found::NotFound, post::Post, posts::Posts};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Routes {
    #[at("/post/:id")]
    Post { id: usize },
    #[at("/posts")]
    Posts,
    #[at("/links")]
    Links,
    #[at("/")]
    Home,
    #[at("/pages/*path")]
    Dynamic { path: String },
    #[not_found]
    #[at("/not_found")]
    NotFound,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Post { id } => html! {<Post id={id} />},
        Routes::Posts => {
            html! {<Posts />}
        }
        Routes::Home => html! {<Redirect<Routes> to={Routes::Posts} />},
        Routes::NotFound => html! {<NotFound />},
        Routes::Links => html! {<Links />},
        Routes::Dynamic { path } => html! {<Dynamic path={path} />},
    }
}
