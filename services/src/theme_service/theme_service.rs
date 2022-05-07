use super::theme::Theme;
use once_cell::sync::Lazy;

pub struct ThemePool {
    pub theme: Theme,
}

pub struct ThemeService {
    pool: &'static mut Lazy<ThemePool>,
}

const THEME_KEY: &'static str = "THEME";

impl ThemeService {
    pub fn from_storage() -> ThemeService {
        let instance = unsafe {
            ThemeService {
                pool: &mut THEME_POOL,
            }
        };

        instance.mount_on_dom();
        instance
    }

    pub fn mount_on_dom(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        body.set_class_name(self.pool.theme.clone().into_str());
    }
    pub fn get_theme_from_OS() -> Theme {
        let is_dark_theme = web_sys::window()
            .unwrap()
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap()
            .matches();

        if is_dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    pub fn get_theme_from_storage() -> Theme {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        match local_storage.get_item(THEME_KEY).unwrap() {
            Some(theme_literal) => Theme::from(&theme_literal),
            None => {
                // TODO: following OS theme by default
                local_storage
                    .set_item(THEME_KEY, Theme::Light.into_str())
                    .unwrap();

                Theme::Light
            }
        }
    }

    pub fn get_theme(&self) -> &Theme {
        &self.pool.theme
    }

    fn update_theme(&mut self) {
        let theme = ThemeService::get_theme_from_storage();
        self.pool.theme = theme;
        self.mount_on_dom();
    }

    pub fn set_theme(&mut self, theme: &Theme) {
        let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
        let stringify_theme = theme.clone().into_str();

        local_storage.set_item(THEME_KEY, stringify_theme).unwrap();
        self.update_theme();
    }
}

pub static mut THEME_POOL: Lazy<ThemePool> = Lazy::new(|| {
    let theme = ThemeService::get_theme_from_storage();

    ThemePool { theme }
});
