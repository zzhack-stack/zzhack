use cached::proc_macro::cached;
use shared::site_config::Config;

static SITE_CONFIG_STRING: &'static str = include_str!("../../site.config.toml");

#[cached]
pub fn get_site_config() -> Config {
    toml::from_str::<Config>(&SITE_CONFIG_STRING).expect("Wrong format of site.config.toml")
}
