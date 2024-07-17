use anyhow::{anyhow, Result};
use shared::links::LinksConfig;
use std::fs::read_to_string;

const SIDE_CONFIG_PATH: &'static str = "../links.toml";

pub fn get_links_config() -> Result<LinksConfig> {
    let config_string = read_to_string(SIDE_CONFIG_PATH)
        .map_err(|_| anyhow!("Read links failed, cannot find links.toml"))?;
    let config = toml::from_str::<LinksConfig>(&config_string)?;

    Ok(config)
}
