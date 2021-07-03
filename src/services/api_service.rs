use once_cell::sync::Lazy;
use yew::format::Nothing;
use yew::format::Text;
use yew::services::fetch::Request;

pub struct APIService {
    endpoint: &'static str,
    prefix: &'static str,
}

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
        Request::builder()
            .method(method)
            .uri(format!("{}{}{}", self.endpoint, self.prefix, path))
            .header("Content-Type", "application/json")
            .body(body)
            .expect("Request failure")
    }
}

pub static api_service: Lazy<APIService> =
    Lazy::new(|| APIService::new("http://localhost:3000", None));
