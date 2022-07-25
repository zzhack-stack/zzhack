use global::theme_context::ThemeContext;
use material_yew::MatIconButton;
use stylist::{style, yew::styled_component};
use utils::resource::{with_assets, with_assets_by_theme};
use utils::theme::with_reactive_source;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub source: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ThemeImageProps {
    #[prop_or(false)]
    pub is_reactive: bool,
    pub source: &'static str,
    #[prop_or(String::from(""))]
    pub style: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    pub source: &'static str,
    #[prop_or(true)]
    pub has_theme: bool,
    pub size: i32,
    #[prop_or(String::from(""))]
    pub style: String,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BaseImageProps {
    pub source: &'static str,
    #[prop_or(false)]
    pub is_raw_source: bool,
    #[prop_or(false)]
    pub has_theme: bool,
    #[prop_or(false)]
    pub is_reactive: bool,
    #[prop_or(String::from(""))]
    pub style: String,
}

#[function_component(BaseImage)]
pub fn base_image(props: &BaseImageProps) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let source = if props.is_reactive {
        with_reactive_source(props.source.to_string())
    } else {
        props.source.to_string()
    };
    let source = if props.is_raw_source {
        source
    } else if props.has_theme {
        with_assets_by_theme(&source, &theme_ctx.theme)
    } else {
        with_assets(&source)
    };

    html! {
        <img class={props.style.clone()} src={source} />
    }
}

#[function_component(Image)]
pub fn image(props: &ImageProps) -> Html {
    html! {
        <BaseImage source={props.source}  />
    }
}

#[function_component(ThemeImage)]
pub fn theme_image(props: &ThemeImageProps) -> Html {
    html! {
        <BaseImage source={props.source} has_theme=true is_reactive={props.is_reactive} style={props.style.clone()} />
    }
}

#[function_component(ThemeRawImage)]
pub fn theme_raw_image(props: &ThemeImageProps) -> Html {
    html! {
        <BaseImage source={props.source} has_theme=true is_reactive={props.is_reactive} style={props.style.clone()} is_raw_source={true} />
    }
}

#[styled_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
    let style = style!(
        r"
        width: ${size}px;
        height: ${size}px;
    ",
        size = props.size,
    )
    .unwrap();
    let style = style.get_class_name();
    let wrapper_style = style!(
        r"
        --mdc-icon-size: ${size}px;
    ",
        size = props.size,
    )
    .unwrap();
    let onclick_callback = match props.onclick.clone() {
        Some(callback) => callback,
        None => Callback::noop(),
    };

    html! {
        <div onclick={onclick_callback} class={vec![wrapper_style.get_class_name().to_string(), props.style.clone()]}>
            <MatIconButton>
                <BaseImage has_theme={props.has_theme} source={props.source} style={style.to_string()} />
            </MatIconButton>
        </div>
    }
}
