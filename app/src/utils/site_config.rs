use shared::site_config::Config;

#[cfg(target_arch = "wasm32")]
pub fn get_config() -> Config {
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let stringify_site_config = local_storage.get_item("site_config").unwrap().unwrap();

    serde_json::from_str::<Config>(&stringify_site_config).unwrap()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_config() -> Config {
    site_config::get_site_config()
}
