use crate::header::drawer_item::DrawerItem;
use stylist::{css, yew::styled_component};
use utils::html::render_with_insert_node;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DrawerProps {
    pub is_open: UseStateHandle<bool>,
    pub children: ChildrenWithProps<DrawerItem>,
}

#[styled_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let style = css!(
        r"
        width: 100%;
        transition: all 0.2s;
        position: absolute;
        left: 0;
        background: var(--underlay-color);
        z-index: 5;
        transform: translateY(${translate});

        .drawer-items {
            position: relative;
            z-index: 5;
            background: var(--underlay-color);
            padding: 0 20px;
        }

        .line {
            width: 100%;
            height: 1px;
            background: var(--shallow-gray);
        }
    ",
        translate = if *props.is_open { "56px" } else { "-100%" },
    );
    let mask_style = css!(
        r"
        top: 0;
        left: 0;
        position: fixed;
        width: 100%;
        height: 100vh;
        background: var(--mask-color);
        z-index: 4;
        display: ${display};
    ",
        display = if *props.is_open { "block" } else { "none" }
    );
    let render_nodes = props
        .children
        .iter()
        .map(|item| html! {{item}})
        .collect::<Vec<Html>>();
    let handle_mask_click = {
        let is_open = props.is_open.clone();

        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <>
            <div class={mask_style} onclick={handle_mask_click} />
            <div class={style}>
                <div class="drawer-items">
                    {render_with_insert_node(&render_nodes, &html! {
                        <div class="line" />
                    })}
                </div>
            </div>
        </>
    }
}
