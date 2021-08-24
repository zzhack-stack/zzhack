use crate::services::api_service::api_service;
use crate::services::APIService;
use crate::FetchTask;
use crate::Res;
use once_cell::sync::Lazy;
use serde::Deserialize;
use yew::services::fetch::Request;
use yew::services::FetchService;
use yew::Callback;

pub struct GitHubService {
    api: APIService,
}

#[derive(Deserialize)]
pub struct GitHubProfile {
    pub login: String,
    pub avatar_url: String,
    pub name: String,
}

impl GitHubService {
    fn new() -> GitHubService {
        GitHubService {
            api: APIService::new("https://api.github.com", None),
        }
    }

    pub fn get_profile(
        &self,
        username: &'static str,
        callback: Callback<Res<GitHubProfile>>,
    ) -> FetchTask {
        let request = self.api.get(format!("/users/{}", username));

        FetchService::fetch(request, callback).unwrap()
    }
}

static github_service: Lazy<GitHubService> = Lazy::new(|| GitHubService::new());
