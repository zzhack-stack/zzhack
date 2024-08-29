use crate::utils::theme::{parse_theme_icon, with_class_prop};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThemeImgProps {
    pub src: String,
    #[prop_or_default]
    pub class: Option<String>,
    pub alt: String,
}

#[function_component]
pub fn ThemeImg(props: &ThemeImgProps) -> Html {
    let theme_icon = parse_theme_icon(&props.src);

    html! {
        <>
            <img
                alt={props.alt.to_string()}
                class={with_class_prop("hidden dark:block", &props.class)}
                src={theme_icon.dark}
            />
            <img
                alt={props.alt.to_string()}
                class={with_class_prop("block dark:hidden", &props.class)}
                src={theme_icon.light}
            />
        </>
    }
}
