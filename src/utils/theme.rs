use crate::services::theme_service::{
    ThemeService,
    DARK_THEME_KEY,
};

pub fn by_theme<T>(light: T, dark: T) -> T {
    let theme = ThemeService::get_theme();

    if theme == DARK_THEME_KEY {dark} else {light}
}