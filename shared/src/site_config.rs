use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct Config {
    pub root: RootConfig,
    pub server: ServerConfig,
    pub nav: Vec<NavConfig>,
}

#[derive(PartialEq, Serialize, Eq, Debug, Deserialize, Clone)]
pub struct NavConfig {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RootConfig {
    pub posts_folder_name: String,
    pub dynamic_pages_folder_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct ServerConfig {
    pub dev_port: usize,
    pub prod_port: usize,
}
