use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LinksConfig {
    pub links: Vec<Link>,
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub name: String,
    pub description: String,
    pub url: String,
    pub avatar: String,
}
