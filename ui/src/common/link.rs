use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkProps {
    pub href: &'static str,
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

    html! {
        <a class={style} href={props.href}>
            {props.children.clone()}
        </a>
    }
}
