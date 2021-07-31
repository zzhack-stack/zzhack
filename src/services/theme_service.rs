pub struct ThemeService {
    pub theme: String,
}

const THEME_KEY: &'static str = "theme";
pub type Theme = &'static str;
pub const LIGHT_THEME_KEY: Theme = "light";
pub const DARK_THEME_KEY: Theme = "dark";
pub const DEFAULT_THEME_KEY: Theme = LIGHT_THEME_KEY;

impl ThemeService {
    pub fn new() -> ThemeService {
        let theme = ThemeService::get_theme();

        ThemeService { theme }
    }

    pub fn init() {
        let theme = ThemeService::get_theme();

        ThemeService::set_theme(theme.as_str());
    }

    pub fn get_theme() -> String {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        match local_storage.get_item(THEME_KEY).unwrap() {
            Some(theme) => theme,
            None => {
                // TODO: following OS theme by default
                local_storage
                    .set_item(THEME_KEY, DEFAULT_THEME_KEY)
                    .unwrap();

                DEFAULT_THEME_KEY.to_string()
            }
        }
    }

    pub fn set_theme(theme: &str) {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        local_storage.set_item(THEME_KEY, theme).unwrap();

        body.set_class_name(theme);
    }

    pub fn switch(&mut self, is_dark: bool) {
        let theme = if is_dark {
            DARK_THEME_KEY
        } else {
            LIGHT_THEME_KEY
        };

        self.theme = theme.to_string();

        ThemeService::set_theme(theme);
    }
}
