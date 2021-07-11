use crate::services::api_service::api_service;
use crate::services::api_service::Res;
use once_cell::sync::Lazy;
use serde::Deserialize;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;
use yew::services::FetchService;
use yew::Callback;

pub struct ArticleService {
    base_path: &'static str,
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Label {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Chapter {
    pub content: String,
    pub number: u32,
    pub articles: Vec<Article>,
    pub title: String,
    pub user: User,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Book {
    pub content: String,
    pub title: String,
    pub number: u32,
    pub user: User,
    pub chapters: Vec<Chapter>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Article {
    pub labels: Vec<Label>,
    pub body: String,
    pub number: u32,
    pub title: String,
    pub user: User,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QueryRes<D> {
    pub incomplete_results: bool,
    pub items: Vec<D>,
    pub total_count: usize,
}

impl ArticleService {
    fn new() -> ArticleService {
        ArticleService {
            base_path: "/search/issues?q=repo:youncccat/blog-database/",
            articles: vec![],
        }
    }

    pub fn sync_articles(&self, callback: Callback<Res<QueryRes<Article>>>) -> FetchTask {
        let request = api_service.get(self.base_path.to_string());

        FetchService::fetch(request, callback).unwrap()
    }

    pub fn set_articles(&mut self, articles: Vec<Article>) {
        self.articles = articles;
    }

    pub fn get_articles_by_label(&self, target_label: String) -> Vec<Article> {
        let mut articles: Vec<Article> = Vec::new();

        for article in self.articles.iter() {
            if match article
                .labels
                .iter()
                .find(|label| label.name == target_label)
            {
                Some(label) => true,
                None => false,
            } {
                articles.push(article.clone())
            }
        }

        articles
    }

    pub fn get_article_by_label(&self, target_label: &'static str) -> Option<Article> {
        match self.articles.iter().find(|article| {
            match article
                .labels
                .iter()
                .find(|label| label.name == target_label)
            {
                Some(_) => true,
                None => false,
            }
        }) {
            Some(article) => Some(article.clone()),
            None => None,
        }
    }

    pub fn get_book_by_number(&self, issue_number: u32) -> Book {
        let get_related_label = |number: u32| -> String { format!("related:{}", number) };
        let book = self
            .articles
            .iter()
            .find(|article| article.number == issue_number)
            .unwrap();
        let chapters = self
            .get_articles_by_label(get_related_label(issue_number))
            .iter()
            .map(|chapter| Chapter {
                number: chapter.number,
                user: chapter.user.clone(),
                title: chapter.title.clone(),
                content: chapter.body.clone(),
                articles: self.get_articles_by_label(get_related_label(chapter.number)),
            })
            .collect();

        Book {
            title: book.title.clone(),
            content: book.body.clone(),
            user: book.user.clone(),
            chapters,
            number: book.number,
        }
    }
}

pub static mut article_service: Lazy<ArticleService> = Lazy::new(|| ArticleService::new());
