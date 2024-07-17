use app::portal::{BrowserApp, BrowserAppProps};

fn main() {
    let config = site_config::get_site_config();

    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::Renderer::<BrowserApp>::with_props(BrowserAppProps { config }).hydrate();
}
