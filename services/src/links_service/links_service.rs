use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json;
#[derive(Deserialize, Clone, PartialEq)]
pub struct RawLinkData {
    pub links: Vec<LinkData>,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct LinkData {
    pub name: String,
    pub desc: String,
    pub addr: String,
    pub logo: Option<String>,
}

pub struct LinksService {
    links_data: Vec<LinkData>,
}

impl LinksService {
    pub fn new() -> LinksService {
        let links_data = include_str!("./links.json");
        let links_data: RawLinkData = serde_json::from_str(links_data).unwrap();

        LinksService {
            links_data: links_data.links,
        }
    }

    pub fn get_links_data(&self) -> Vec<LinkData> {
        self.links_data.clone()
    }
}

pub static LINKS_SERVICE: Lazy<LinksService> = Lazy::new(|| LinksService::new());
