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
}

#[derive(Deserialize, Debug, Clone)]
pub struct Label {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Article {
    pub labels: Vec<Label>,
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QueryRes<D> {
    pub incomplete_results: bool,
    pub items: Vec<D>,
    pub total_count: usize,
}

impl ArticleService {
    pub fn new() -> ArticleService {
        ArticleService {
            base_path: "/search/issues?q=repo:youncccat/blog-database/",
        }
    }

    pub fn get_all(&self, callback: Callback<Res<Vec<Article>>>) -> FetchTask {
        let request = api_service.get(self.base_path.to_string());

        FetchService::fetch(request, callback).unwrap()
    }

    pub fn get_article_by_label(
        &self,
        label: &'static str,
        callback: Callback<Res<QueryRes<Article>>>,
    ) -> FetchTask {
        let request = api_service.get(format!("{}+label:{}", self.base_path.to_string(), label));

        FetchService::fetch(request, callback).unwrap()
    }
}

pub static article_service: Lazy<ArticleService> = Lazy::new(|| ArticleService::new());
