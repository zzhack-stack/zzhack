use crate::global::theme_context::Theme;
use crate::services::theme_service::ThemeService;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub fn by_theme<T>(light: T, dark: T) -> T {
    let theme_service = ThemeService::from_storage();
    let theme = theme_service.get_theme();

    if theme == &Theme::Dark {
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

pub fn only_render_on_pc(vnode: VNode) -> VNode {
    if is_on_mobile() {
        html! {}
    } else {
        vnode
    }
}

pub fn only_render_on_mobile(vnode: VNode) -> VNode {
    if is_on_mobile() {
        vnode
    } else {
        html! {}
    }
}

pub fn by_reactive<T>(mobile: T, pc: T) -> T {
    if is_on_mobile() {
        mobile
    } else {
        pc
    }
}
