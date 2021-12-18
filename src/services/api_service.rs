use once_cell::sync::Lazy;
use regex::Regex;
use yew::format::Json;
use yew::format::Nothing;
use yew::services::fetch::Request;
use yew::services::fetch::Response;

pub struct APIService {
    endpoint: &'static str,
    prefix: &'static str,
}

pub type Res<R> = Response<Json<Result<R, anyhow::Error>>>;
pub type Headers<'a> = Option<Vec<(&'a str, &'a str)>>;

impl APIService {
    pub fn new(endpoint: &'static str, prefix: Option<&'static str>) -> APIService {
        let prefix = match prefix {
            Some(prefix) => prefix,
            None => "",
        };

        APIService { endpoint, prefix }
    }

    pub fn get(&self, path: String) -> Request<Nothing> {
        self.fetch(path, Nothing, "get", None)
    }

    pub fn post<T>(&self, path: String, body: T) -> Request<Json<T>> {
        self.fetch(path, Json(body), "post", None)
    }

    pub fn post_with_headers<T>(
        &self,
        path: String,
        body: T,
        headers: Headers,
    ) -> Request<Json<T>> {
        self.fetch(path, Json(body), "post", headers)
    }

    pub fn get_with_headers(&self, path: String, headers: Headers) -> Request<Nothing> {
        self.fetch(path, Nothing, "get", headers)
    }

    fn fetch<B>(
        &self,
        path: String,
        body: B,
        method: &'static str,
        headers: Headers,
    ) -> Request<B> {
        let regex = Regex::new("^http").unwrap();
        let parsed_uri = if regex.is_match(path.as_str()) {
            path
        } else {
            format!("{}{}{}", self.endpoint, self.prefix, path)
        };

        let mut request = Request::builder()
            .method(method)
            .uri(parsed_uri)
            .header("Content-Type", "application/json");

        let request = match headers {
            Some(headers) => {
                for header in headers {
                    let (key, value) = header;

                    request = request.header(key, value);
                }

                request
            }
            None => request,
        };

        request.body(body).expect("Request failure")
    }
}

pub static api_service: Lazy<APIService> =
    Lazy::new(|| APIService::new("http://localhost:3000", None));
