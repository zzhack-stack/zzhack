use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum RootRoutes {
    #[at("/home")]
    Home,
    #[at("/posts/:filename")]
    Post { filename: String },
    #[at("/")]
    Root,
    #[at("/projects")]
    Projects,
    #[at("/links")]
    Links,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}
