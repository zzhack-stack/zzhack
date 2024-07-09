use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, not_found::NotFound, post::Post, posts::Posts};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Routes {
    #[at("/posts/:id")]
    Post { id: u32 },
    #[at("/posts")]
    Posts,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Post { id } => html! {<Post />},
        Routes::Posts => {
            html! {<Posts />}
        }
        Routes::Home => html! {<Home />},
        Routes::NotFound => html! {<NotFound />},
    }
}
