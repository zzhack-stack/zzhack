use crate::link::Link;
use router::RootRoutes;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DrawerProps {
    pub lnk: RootRoutes,
    pub children: Children,
}

#[function_component(DrawerItem)]
pub fn drawer_item(props: &DrawerProps) -> Html {
    let style = use_style!(
        r"
        padding: 12px 0;
    "
    );

    html! {
        <div class={style}>
            <Link href={props.lnk.clone()}>{props.children.clone()}</Link>
        </div>
    }
}
