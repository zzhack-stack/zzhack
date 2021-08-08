use once_cell::sync::Lazy;
use regex::Regex;
use yew::format::Json;
use yew::format::Nothing;
use yew::format::Text;
use yew::services::fetch::Request;
use yew::services::fetch::Response;
use yew::Callback;

pub struct APIService {
    endpoint: &'static str,
    prefix: &'static str,
}

pub type Res<R> = Response<Json<Result<R, anyhow::Error>>>;

impl APIService {
    fn new(endpoint: &'static str, prefix: Option<&'static str>) -> APIService {
        let prefix = match prefix {
            Some(prefix) => prefix,
            None => "",
        };

        APIService { endpoint, prefix }
    }

    pub fn get(&self, path: String) -> Request<Nothing> {
        self.fetch(path, Nothing, "get")
    }

    fn fetch<B>(&self, path: String, body: B, method: &'static str) -> Request<B>
    where
        B: Into<Text> + std::fmt::Debug,
    {
        let regex = Regex::new("^http").unwrap();
        let parsed_uri = if regex.is_match(path.as_str()) {
            format!("{}{}{}", self.endpoint, self.prefix, path)
        } else {
            path
        };

        Request::builder()
            .method(method)
            .uri(parsed_uri)
            .header("Content-Type", "application/json")
            .body(body)
            .expect("Request failure")
    }
}

pub static api_service: Lazy<APIService> =
    Lazy::new(|| APIService::new("https://api.github.com", None));
