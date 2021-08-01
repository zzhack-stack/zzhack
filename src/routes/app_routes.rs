use crate::article_service;
use crate::console_log;
use crate::pages::book::BookView;
use crate::pages::oauth_redirect::OAuthRedirect;
use crate::pages::{
    about_me::AboutMe, article::ArticleView, articles::Articles, home::Home, not_found::NotFound,
    technology::Technology,
};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoutes {
    #[to = "/oauth/redirect?code={code}&origin={url}"]
    GitHubOAuthRedirect(String, String),
    #[to = "/articles/{number}"]
    Articles(u32),
    #[to = "/books/{book_number}/chapters/{chapter_number}/articles/{article_number}"]
    BooksWithArticle(u32, u32, u32),
    #[to = "/books/{book_number}/chapters/{chapter_number}"]
    BooksWithChapter(u32, u32),
    #[to = "/books/{number}"]
    Books(u32),
    #[to = "/about"]
    About,
    #[to = "/technology"]
    Technology,
    #[to = "/thinking"]
    Thinking,
    #[to = "/404"]
    NotFound,
    // #[to = "/"; redirect = ""]
    // Home,
}

pub type AppRouterAnchor = RouterAnchor<AppRoutes>;

pub fn switch(routes: AppRoutes) -> Html {
    match routes {
        AppRoutes::GitHubOAuthRedirect(code, redirect_url) => {
            html! {<OAuthRedirect code={code} redirect_url=redirect_url />}
        }
        AppRoutes::About => {
            html! { <AboutMe />}
        }
        AppRoutes::NotFound => {
            html! {<NotFound />}
        }
        AppRoutes::Technology => {
            html! {<Technology />}
        }
        AppRoutes::BooksWithArticle(book_number, chapter_number, article_number) => {
            html! { <BookView number=book_number chapter_number=chapter_number article_number=article_number  />}
        }
        AppRoutes::BooksWithChapter(book_number, chapter_number) => {
            html! { <BookView number=book_number chapter_number=chapter_number  />}
        }
        AppRoutes::Books(number) => {
            html! { <BookView number=number />}
        }
        AppRoutes::Articles(number) => {
            let article = unsafe { article_service.get_article_by_number(number) };

            html! {<ArticleView article=article />}
        }
        // AppRoutes::Home => {
        //     html! { <Home />}
        // }
        _ => html! {<NotFound />},
    }
}
