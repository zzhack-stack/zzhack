use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    #[prop_or("")]
    pub href: &'static str,
    #[prop_or(String::from(""))]
    pub dynamic_href: String,
    pub children: Children,
}

#[function_component(Link)]
pub fn link(props: &LinkProps) -> Html {
    let style = use_style!(
        r"
        text-decoration: none;
        transition: 0.3s opacity;
        
        &:hover {
            opacity: 0.75;
        }
    "
    );
    let target = if props.dynamic_href != "" {
        props.dynamic_href.clone()
    } else {
        String::from(props.href)
    };

    html! {
        <a class={style} href={target}>
            {props.children.clone()}
        </a>
    }
}
