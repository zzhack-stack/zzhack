use crate::console_log;
use crate::services::theme_service::{ThemeService, DARK_THEME_KEY};

pub fn by_theme<T>(light: T, dark: T) -> T {
    let theme = ThemeService::get_theme();

    if theme == DARK_THEME_KEY {
        dark
    } else {
        light
    }
}

pub fn is_on_mobile() -> bool {
    web_sys::window()
        .unwrap()
        .match_media("(max-width: 600px)")
        .unwrap()
        .unwrap()
        .matches()
}

pub fn by_reactive<T>(mobile: T, pc: T) -> T {
    console_log!("{}", is_on_mobile());

    if is_on_mobile() {
        mobile
    } else {
        pc
    }
}
