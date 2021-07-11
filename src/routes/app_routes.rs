use crate::console_log;
use crate::pages::book::BookView;
use crate::pages::{about_me::AboutMe, home::Home, not_found::NotFound, technology::Technology};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoutes {
    #[to = "/books/{book_number}/chapters/{chapter_number}/articles/{article_number}"]
    BooksWithArticle(u32, u32, u32),
    #[to = "/books/{book_number}/chapters/{chapter_number}"]
    BooksWithChapter(u32, u32),
    #[to = "/books/{number}"]
    Books(u32),
    #[to = "/about/me"]
    AboutMe,
    #[to = "/technology"]
    Technology,
    #[to = "/thinking"]
    Thinking,
    #[to = "/404"]
    NotFound,
    #[to = "/"]
    Home,
    #[to = "/technology/articles"]
    Articles,
    #[to = "/technology/open_source"]
    OpenSource,
}

pub type AppRouterAnchor = RouterAnchor<AppRoutes>;

pub fn switch(routes: AppRoutes) -> Html {
    match routes {
        AppRoutes::AboutMe => {
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
        AppRoutes::Home => {
            html! { <Home />}
        }
        _ => html! {<NotFound />},
    }
}
