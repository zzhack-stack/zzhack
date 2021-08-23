use crate::services::api_service::api_service;
// use crate::services::api_service::Request;
use crate::services::APIService;
use crate::FetchTask;
use crate::Res;
use once_cell::sync::Lazy;
use yew::services::FetchService;
use yew::Callback;

pub struct GitHubService {
    api: APIService,
}

impl GitHubService {
    fn new() -> GitHubService {
        GitHubService {
            api: APIService::new("https://api.github.com", None),
        }
    }

    // fn send_request<T>(&self, request: Request,callback: Callback<Res<T>>) -> FetchTask {
    // FetchService::fetch(request, callback).unwrap()
    // }

    fn get_profile(&self, username: &'static str) {}
}

static github_service: Lazy<GitHubService> = Lazy::new(|| GitHubService::new());
