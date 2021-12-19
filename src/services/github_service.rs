use crate::services::APIService;
use crate::utils::request::JSONCallback;
use crate::CacheService;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use yew::services::fetch::FetchTask;
use yew::services::FetchService;
use yew::Callback;

pub struct GitHubService {
    github_api: APIService,
    github_raw_api: APIService,
    oauth_server_api: APIService,
}

#[derive(Deserialize)]
pub struct GitHubAccessKey {
    pub access_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GitHubProfile {
    pub login: String,
    pub avatar_url: String,
    pub name: Option<String>,
    pub bio: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubIssueUser {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubIssueComment {
    pub body: String,
    pub user: GitHubIssueUser,
    pub created_at: String,
    pub updated_at: String,
}

pub type GitHubIssueComments = Vec<GitHubIssueComment>;

const GITHUB_OAUTH_CLIENT_ID: &'static str = "1907130b302d4b7c0176";

impl JSONCallback for GitHubService {}

impl GitHubService {
    pub fn new() -> GitHubService {
        GitHubService {
            github_api: APIService::new("https://api.github.com", None),
            github_raw_api: APIService::new("https://github.com", None),
            oauth_server_api: APIService::new("https://zzhack-oauth.vercel.app", None),
        }
    }

    pub fn fetch_token(&mut self, code: &str, callback: Callback<Option<String>>) -> FetchTask {
        let request = self.oauth_server_api.get(format!(
            "/api/access_token?code={}&client_id={}",
            code, GITHUB_OAUTH_CLIENT_ID
        ));

        FetchService::fetch(
            request,
            GitHubService::wrap_callback(Callback::from(move |data: GitHubAccessKey| {
                let access_token = data.access_token;
                let cache_service = CacheService::new();
                match &access_token {
                    Some(access_token) => {
                        cache_service.set_github_access_key(access_token.as_str())
                    }
                    None => {}
                };

                callback.emit(access_token)
            })),
        )
        .unwrap()
    }

    pub fn create_issue(
        &self,
        issue_number: i32,
        body: &str,
        callback: Callback<GitHubIssueComment>,
    ) -> FetchTask {
        let access_token = CacheService::new().get_github_access_key().unwrap();
        let body = json!({ "body": body });
        let token_header_value = format!("token {}", access_token);
        let request = self.github_api.post_with_headers(
            format!(
                "/repos/zzhack-stack/zzhack-provider/issues/{}/comments",
                issue_number
            ),
            &body,
            Some(vec![("Authorization", token_header_value.as_str())]),
        );

        FetchService::fetch(request, GitHubService::wrap_callback(callback)).unwrap()
    }

    pub fn get_user_profile(&self, callback: Callback<GitHubProfile>) -> Option<FetchTask> {
        let access_token = match CacheService::new().get_github_access_key() {
            Some(access_token) => access_token,
            None => return None,
        };
        let token_header_value = format!("token {}", access_token);
        let request = self.github_api.get_with_headers(
            String::from("/user"),
            Some(vec![("Authorization", token_header_value.as_str())]),
        );

        Some(
            FetchService::fetch(
                request,
                GitHubService::wrap_callback(Callback::from(move |profile: GitHubProfile| {
                    CacheService::new().set_github_profile(&profile);
                    callback.emit(profile);
                })),
            )
            .unwrap(),
        )
    }

    pub fn get_issue_comments(
        &self,
        issue_number: i32,
        callback: Callback<GitHubIssueComments>,
    ) -> FetchTask {
        let request = self.github_api.get(format!(
            "/repos/zzhack-stack/zzhack-provider/issues/{}/comments",
            issue_number
        ));

        FetchService::fetch(
            request,
            GitHubService::wrap_callback(Callback::from(move |comments: GitHubIssueComments| {
                let mut comments = comments;

                comments.reverse();
                callback.emit(comments);
            })),
        )
        .unwrap()
    }

    pub fn get_oauth_url() -> String {
        let href = web_sys::window().unwrap().location().href().unwrap();

        format!(
            "https://github.com/login/oauth/authorize?redirect_uri=https://zzhack.fun/oauth/redirect?origin={}&client_id={}&scope=public_repo",
            href, GITHUB_OAUTH_CLIENT_ID
        )
    }
}

static github_service: Lazy<GitHubService> = Lazy::new(|| GitHubService::new());
