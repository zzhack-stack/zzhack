use cached::proc_macro::cached;
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;

const SIDE_CONFIG_PATH: &'static str = "../site.config.toml";

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

#[cached]
pub fn get_site_config() -> Config {
    let config_string = read_to_string(SIDE_CONFIG_PATH).expect(
        "Cannot find site.config.toml, please make sure you have one in the root directory.",
    );

    toml::from_str::<Config>(&config_string).expect("Wrong format of site.config.toml")
}
