use serde_derive::Deserialize;
use std::fs::read_to_string;

const SIDE_CONFIG_PATH: &'static str = "../site.config.toml";

#[derive(Deserialize)]
pub struct Config {
    pub root: RootConfig,
}

#[derive(Deserialize)]
pub struct RootConfig {
    pub posts_folder_name: String,
}

pub fn get_site_config() -> RootConfig {
    let config_string = read_to_string(SIDE_CONFIG_PATH).expect(
        "Cannot find site.config.toml, please make sure you have one in the root directory.",
    );

    toml::from_str::<Config>(&config_string)
        .expect("Wrong format of site.config.toml")
        .root
}
