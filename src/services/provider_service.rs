use crate::services::APIService;
use crate::FetchTask;
use crate::Res;
use core::ops::Index;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::Path;
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

#[derive(Deserialize, Clone, Serialize)]
pub struct PostMetadata {
    pub title: String,
    pub summary: String,
    pub cover: String,
    pub create_at: u64,
    pub filename: String,
    pub content: String,
    pub issue_id: i32,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Categories {
    pub technology: Vec<PostMetadata>,
    pub thinking: Vec<PostMetadata>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct RootMetadata {
    pub categories: Categories,
}

#[derive(Deserialize, Clone)]
pub struct Fragment {
    pub cover: String,
    pub create_at: u64,
    pub content: String,
}

#[derive(Deserialize, Clone)]
pub struct Fragments {
    pub fragments: Vec<Fragment>,
}

impl Index<&'_ str> for Categories {
    type Output = Vec<PostMetadata>;
    fn index(&self, category_name: &str) -> &Vec<PostMetadata> {
        match category_name {
            "technology" => &self.technology,
            "thinking" => &self.thinking,
            _ => panic!(format!(
                "Cannot find the {} of value from the categories",
                category_name
            )),
        }
    }
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
            let Json(body) = res.into_body();
            let data = body.unwrap();

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

    fn get_post_filename<'a>(&self, filename: &'a str) -> &'a str {
        let post_resolver = Path::new(filename);
        match post_resolver.file_stem().and_then(OsStr::to_str) {
            Some(filename) => filename,
            None => panic!("Cannot find the name of this file."),
        }
    }

    pub fn get_post_metadata<'a>(
        &self,
        category: &'a str,
        post_filename: &'a str,
        callback: Callback<PostMetadata>,
    ) -> FetchTask {
        let filename = self.get_post_filename(post_filename);
        let request = self
            .api
            .get(format!("/posts/{}/{}/metadata.json", category, filename));

        FetchService::fetch(request, ProviderService::wrap_callback(callback)).unwrap()
    }

    pub fn get_fragments(&self, callback: Callback<Fragments>) -> FetchTask {
        let request = self.api.get(String::from("/fragments.json"));

        FetchService::fetch(request, ProviderService::wrap_callback(callback)).unwrap()
    }
}

pub static provider_service: Lazy<ProviderService> = Lazy::new(|| ProviderService::new());
