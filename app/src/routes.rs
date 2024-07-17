use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, links::Links, not_found::NotFound, post::Post, posts::Posts};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Routes {
    #[at("/post/:id")]
    Post { id: usize },
    #[at("/posts")]
    Posts,
    #[at("/links")]
    Links,
    #[not_found]
    #[at("/not_found")]
    NotFound,
    #[at("/")]
    Home,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Post { id } => html! {<Post id={id} />},
        Routes::Posts => {
            html! {<Posts />}
        }
        Routes::Home => html! {<Home />},
        Routes::NotFound => html! {<NotFound />},
        Routes::Links => html! {<Links />},
    }
}
