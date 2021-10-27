use crate::services::APIService;
use crate::FetchTask;
use crate::Res;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use yew::format::Json;
use yew::services::FetchService;
use yew::Callback;

pub struct ProviderService {
    api: APIService,
}

#[derive(Deserialize, Debug)]
pub struct PinnedProject {
    pub title: String,
    pub desc: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct PinnedProjects {
    pub projects: Vec<PinnedProject>,
}

#[derive(Deserialize, Clone)]
pub struct PostMetadata {
    pub title: String,
    pub summary: String,
    pub cover: String,
    pub create_at: i64,
    pub filename: String,
}

#[derive(Deserialize, Clone)]
pub struct Categories {
    pub technology: Vec<PostMetadata>,
    pub thinking: Vec<PostMetadata>,
    pub fragments: Vec<PostMetadata>,
}

#[derive(Deserialize, Clone)]
pub struct RootMetadata {
    pub categories: Categories,
}

impl ProviderService {
    pub fn new() -> ProviderService {
        ProviderService {
            api: APIService::new("https://cdn.ahlele.com", None),
        }
    }

    fn wrap_callback<'a, T: 'static>(callback: Callback<T>) -> Callback<Res<T>>
    where
        T: DeserializeOwned,
    {
        Callback::from(move |res: Res<T>| {
            let Json(data) = res.into_body();
            let data = data.unwrap();

            callback.emit(data)
        })
    }

    pub fn get_pinned_projects(&self, callback: Callback<PinnedProjects>) -> FetchTask {
        let request = self
            .api
            .get(String::from("/constants/pinned_projects.json"));

        FetchService::fetch(request, ProviderService::wrap_callback(callback)).unwrap()
    }

    pub fn get_root_metadata(&self, callback: Callback<RootMetadata>) -> FetchTask {
        let request = self.api.get(String::from("/metadata.json"));

        FetchService::fetch(request, ProviderService::wrap_callback(callback)).unwrap()
    }
}

pub static provider_service: Lazy<ProviderService> = Lazy::new(|| ProviderService::new());
