use reqwest::Response;

pub struct HTTP {
    base_url: String,
}

impl HTTP {
    pub fn new() -> HTTP {
        HTTP {
            base_url: Self::get_base_url("/api"),
        }
    }

    #[cfg(debug_assertions)]
    fn get_port() -> usize {
        site_config::get_site_config().server.dev_port
    }

    #[cfg(not(debug_assertions))]
    fn get_port() -> usize {
        site_config::get_site_config().server.prod_port
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_base_url(prefix: &'static str) -> String {
        format!("http://localhost:{}{}", Self::get_port(), prefix)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_base_url(prefix: &'static str) -> String {
        web_sys::window().unwrap().origin()
    }

    fn with_base_url(&self, path: &str) -> String {
        format!("{}{path}", self.base_url)
    }

    pub async fn get(&self, path: &str) -> reqwest::Result<Response> {
        reqwest::get(self.with_base_url(path)).await
    }
}
