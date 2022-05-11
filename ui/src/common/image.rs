use global::theme_context::ThemeContext;
use material_yew::MatIconButton;
use stylist::{style, yew::styled_component};
use utils::resource::{with_assets, with_assets_by_theme};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ImageProps {
    pub source: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ThemeImageProps {
    pub source: &'static str,
}

#[derive(Properties, Clone, PartialEq)]
pub struct IconProps {
    pub source: &'static str,
    #[prop_or(true)]
    pub has_theme: bool,
    pub size: i32,
    #[prop_or(String::from(""))]
    pub style: String,
}

#[derive(Properties, Clone, PartialEq)]
pub struct BaseImageProps {
    pub source: &'static str,
    #[prop_or(false)]
    pub has_theme: bool,
    #[prop_or(String::from(""))]
    pub style: String,
}

#[function_component(BaseImage)]
fn base_image(props: &BaseImageProps) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let source = if props.has_theme {
        with_assets_by_theme(props.source, &theme_ctx.theme)
    } else {
        with_assets(props.source)
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
        <BaseImage source={props.source} has_theme=true />
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

    html! {
        <div class={vec![wrapper_style.get_class_name().to_string(), props.style.clone()]}>
            <MatIconButton>
                <BaseImage has_theme={props.has_theme} source={props.source} style={style.to_string()} />
            </MatIconButton>
        </div>
    }
}
