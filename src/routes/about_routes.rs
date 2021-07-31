use crate::article_service;
use crate::pages::article::ArticleView;
use crate::pages::book::BookView;
use crate::pages::home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AboutRoutes {
    #[to = "/about/me"]
    AboutMe,
    #[to = "/about/help"]
    AboutHelp,
}

pub type AboutRouterAnchor = RouterAnchor<AboutRoutes>;

pub fn switch(routes: AboutRoutes) -> Html {
    match routes {
        AboutRoutes::AboutMe => {
            let article = unsafe { article_service.get_article_by_number(3) };
            html! { <ArticleView article=article />}
        }
        AboutRoutes::AboutHelp => {
            let article = unsafe { article_service.get_article_by_number(2) };

            html! { <ArticleView article=article />}
        }
        _ => html! {<Home />},
    }
}

pub type AboutRouter = Router<AboutRoutes>;
