use home::Home;
use links::Links;
use not_found::NotFound;
use post::Post;
use projects::Projects;
use router::RootRoutes;
use yew::prelude::*;
use yew_router::prelude::*;

pub fn switch(routes: &RootRoutes) -> Html {
    match routes {
        RootRoutes::Home | RootRoutes::Root => html! { <Home posts_key={String::from("1")} /> },
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::Post { filename } => html! {<Post filename={filename.clone()} />},
        RootRoutes::NotFound => html! { <NotFound />},
        RootRoutes::Technology => html! {
            <Redirect<RootRoutes> to={RootRoutes::Home}/>
        },
        RootRoutes::Links => html! {<Links />},
    }
}
