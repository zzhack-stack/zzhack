use crate::services::api_service::api_service;
use serde::Deserialize;
use yew::format::Json;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;
use yew::services::FetchService;
use yew::Callback;

pub struct ArticleService {}

#[derive(Deserialize, Debug, Clone)]
pub struct Article {
    content: String,
}

impl ArticleService {
    pub fn get(
        path: &'static str,
        callback: Callback<Response<Result<String, anyhow::Error>>>,
    ) -> FetchTask {
        let request = api_service.get(format!("/articles{}", path));

        FetchService::fetch(request, callback).unwrap()
    }
}
