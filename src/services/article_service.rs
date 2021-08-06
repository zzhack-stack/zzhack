use crate::console_log;
use crate::services::api_service::api_service;
use crate::services::api_service::Res;
use once_cell::sync::Lazy;
use regex::Regex;
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
    pub cover: Option<Cover>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Book {
    pub content: String,
    pub title: String,
    pub number: u32,
    pub user: User,
    pub chapters: Vec<Chapter>,
    pub cover: Option<Cover>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Article {
    pub labels: Vec<Label>,
    pub body: String,
    pub number: u32,
    pub title: String,
    pub user: User,
    pub cover: Option<Cover>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct ArticleWithMetadata {
    pub is_book: bool,
    pub article: Article,
    pub filters: Vec<Label>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct Cover {
    pub number: u32,
    pub cover: String,
    pub background: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Covers {
    pub covers: Vec<Cover>,
}

const COVER_ISSUE_NUMBER: u32 = 7;

impl ArticleService {
    fn new() -> ArticleService {
        ArticleService {
            base_path: "/search/issues?q=repo:youncccat/blog-database/+is:open",
            articles: vec![],
        }
    }

    pub fn sync_articles(&self, callback: Callback<Res<QueryRes<Article>>>) -> FetchTask {
        let request = api_service.get(self.base_path.to_string());

        FetchService::fetch(request, callback).unwrap()
    }

    fn select_cover_by_number(&mut self, number: u32) -> Option<&Article> {
        self.articles.iter().find(|article| {
            console_log!("{} {}", article.number, number);
            article.number == number
        })
    }

    fn attach_cover(&mut self) {
        let covers = self.select_cover_by_number(COVER_ISSUE_NUMBER).unwrap();
        let covers: Covers = match serde_json::from_str(covers.body.trim()) {
            Ok(covers) => covers,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };
        let covers = covers.covers;
        let articles: Vec<Article> = self
            .articles
            .iter()
            .map(
                |article| match covers.iter().find(|item| item.number == article.number) {
                    Some(item) => Article {
                        cover: Some(item.clone()),
                        ..(article.clone())
                    },
                    None => Article {
                        cover: None,
                        ..(article.clone())
                    },
                },
            )
            .collect();

        self.articles = articles;
    }

    pub fn set_articles(&mut self, articles: Vec<Article>) {
        self.articles = articles;

        self.attach_cover()
    }

    pub fn get_articles_by_labels_with_metadata(
        &self,
        target_labels: Vec<&'static str>,
    ) -> Vec<ArticleWithMetadata> {
        self.get_articles_by_labels(target_labels)
            .iter()
            .map(|article| ArticleWithMetadata {
                filters: article
                    .labels
                    .clone()
                    .into_iter()
                    .filter(|label| label.name.starts_with("filter:"))
                    .map(|label| Label {
                        name: label.name[7..].to_string(),
                        ..label
                    })
                    .collect(),
                is_book: match article
                    .labels
                    .iter()
                    .find(|label| label.name == "type:book")
                {
                    Some(_) => true,
                    None => false,
                },
                article: article.clone(),
            })
            .collect()
    }

    pub fn get_articles_by_pattern(&self, regex: Regex) -> Vec<Article> {
        self.articles
            .clone()
            .into_iter()
            .filter(|article| regex.is_match(article.title.as_str()))
            .collect()
    }

    pub fn get_articles_by_labels(&self, target_labels: Vec<&str>) -> Vec<Article> {
        let mut articles: Vec<Article> = Vec::new();

        for article in self.articles.iter() {
            if match article
                .labels
                .iter()
                .find(|label| target_labels.contains(&label.name.as_str()))
            {
                Some(label) => true,
                None => false,
            } {
                articles.push(article.clone())
            }
        }

        articles
    }

    pub fn get_articles_by_label(&self, target_label: &str) -> Vec<Article> {
        self.get_articles_by_labels(vec![target_label])
    }

    pub fn get_article_by_label(&self, target_label: &str) -> Option<Article> {
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

    pub fn get_article_by_number(&self, issue_number: u32) -> Article {
        self.articles
            .clone()
            .into_iter()
            .find(|article| article.number == issue_number)
            .unwrap()
    }

    pub fn get_book_by_number(&self, issue_number: u32) -> Book {
        let get_related_label = |number: u32| -> String { format!("related:{}", number) };
        let book = self
            .articles
            .iter()
            .find(|article| article.number == issue_number)
            .unwrap();
        let chapters = self
            .get_articles_by_label(get_related_label(issue_number).as_str())
            .iter()
            .map(|chapter| Chapter {
                created_at: chapter.created_at.clone(),
                updated_at: chapter.updated_at.clone(),
                cover: chapter.cover.clone(),
                number: chapter.number,
                user: chapter.user.clone(),
                title: chapter.title.clone(),
                content: chapter.body.clone(),
                articles: self.get_articles_by_label(get_related_label(chapter.number).as_str()),
            })
            .collect();

        Book {
            updated_at: book.updated_at.clone(),
            created_at: book.created_at.clone(),
            cover: book.cover.clone(),
            title: book.title.clone(),
            content: book.body.clone(),
            user: book.user.clone(),
            chapters,
            number: book.number,
        }
    }
}

pub static mut article_service: Lazy<ArticleService> = Lazy::new(|| ArticleService::new());
