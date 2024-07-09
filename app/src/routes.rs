use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, not_found::NotFound, post::Post, posts::Posts};

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Routes {
    #[at("/post/:name")]
    Post { name: String },
    #[at("/posts")]
    Posts,
    #[not_found]
    #[at("/not_found")]
    NotFound,
    #[at("/")]
    Home,
}

pub fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Post { name } => html! {<Post name={name} />},
        Routes::Posts => {
            html! {<Posts />}
        }
        Routes::Home => html! {<Home />},
        Routes::NotFound => html! {<NotFound />},
    }
}
