use crate::image::Icon;
use crate::modal::{
    modal_action::{ModalAction, ModalActionProps},
    modal_content::{ModalContent, ModalContentProps},
};
use std::collections::HashMap;
use std::rc::Rc;
use stylist::{css, yew::styled_component};
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

// vchild.props -> {props: enum} -> {props} -> vnode(html)

#[derive(Clone, PartialEq)]
pub enum ModalComponentProps {
    Action(Rc<ModalActionProps>),
    Content(Rc<ModalContentProps>),
}

pub enum ModalComponentType {
    Action,
    Content,
    Title,
}

impl From<ModalActionProps> for ModalComponentProps {
    fn from(props: ModalActionProps) -> Self {
        ModalComponentProps::Action(Rc::new(props))
    }
}

impl From<ModalContentProps> for ModalComponentProps {
    fn from(props: ModalContentProps) -> Self {
        ModalComponentProps::Content(Rc::new(props))
    }
}

#[derive(Clone, PartialEq)]
pub struct ModalVariant {
    pub modal_component_props: ModalComponentProps,
}

impl<T> From<VChild<T>> for ModalVariant
where
    T: Component,
    T::Properties: Clone + Into<ModalComponentProps>,
{
    fn from(child: VChild<T>) -> Self {
        ModalVariant {
            modal_component_props: (*child.props).clone().into(),
        }
    }
}

impl From<ModalVariant> for Html {
    fn from(variant: ModalVariant) -> Self {
        match variant.modal_component_props {
            ModalComponentProps::Action(props) => {
                VComp::new::<ModalAction>(props, NodeRef::default(), None).into()
            }
            ModalComponentProps::Content(props) => {
                VComp::new::<ModalContent>(props, NodeRef::default(), None).into()
            }
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub children: ChildrenRenderer<ModalVariant>,
    pub is_visible: UseStateHandle<bool>,
    pub title: &'static str,
    #[prop_or_default]
    pub subtitle: Option<&'static str>,
}

#[styled_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_host = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap();
    let mut nodes = HashMap::<&'static str, Vec<Html>>::new();

    nodes.insert("action", vec![]);
    nodes.insert("content", vec![]);
    extract_nodes(&props.children, &mut nodes);

    let wrapper_style = css!(
        r#"
        display: ${display} !important;
        width: 100%;
        height: 100vh;
        position: fixed;
        left: 0;
        top: 0;
        z-index: 10;
        display: flex;
        justify-content: center;
        align-items: center;
        transition:1s opacity ease-out;
        opacity: ${opacity};

        .modal-box {
            background: var(--card-color);
            border-radius: 10px;
            width: fit-content;
            padding: 15px 0;
            min-width: 240px;
            position: relative;
            z-index: 3;
            position: relative;
        }

        .modal-title, .modal-content {
            padding: 0 20px;
        }

        .modal-actions {
            display: flex;
            justify-content: flex-end;
        }

        .close-btn {
            position: absolute;
            top: 0;
            right: 0;
            z-index: 10;
            display: none;
        }

        .modal-subtitle {
            font-size: 14px;
            color: var(--sub-text-color);
            font-weight: 400;
        }

        .mask {
            display: ${display} !important;
            width: 100%;
            height: 100%;
            position: absolute;
            left: 0;
            top: 0;
            background: var(--mask-color);
        }

        @media (max-width: 600px) {
            .modal-box {
                margin: 0 30px;
            }

            .modal-subtitle {
                font-size: 12px;
            }

            .close-btn {
                display: block;
            }
        }
    "#,
        opacity = if *props.is_visible { 1 } else { 0 },
        display = if *props.is_visible { "flex" } else { "none" },
    );
    let handle_mask_click = {
        let visible = props.is_visible.clone();

        Callback::from(move |_| visible.set(false))
    };
    let render_subtitle = match props.subtitle {
        Some(subtitle) => html! {
            <div class="modal-subtitle">
                {subtitle}
            </div>
        },
        None => html! {},
    };

    let render_component = html! {
        <div class={wrapper_style}>
            <div class="mask" onclick={handle_mask_click.clone()} />
            <div class="modal-box" onclick={Callback::from(|e:MouseEvent| e.prevent_default())}>
                <div class="close-btn">
                    <Icon source="close_btn.svg" size={20} onclick={handle_mask_click.clone()} />
                </div>
                <h2 class="modal-title">
                    {props.title}
                    {render_subtitle}
                </h2>
                <div class="modal-content">
                    { get_single_ele_from_vec_by_default(&nodes, "content") }
                </div>
                <div class="modal-actions">
                    { nodes.get("action").unwrap().into_iter().map(|vnode| vnode.clone()).collect::<Html>() }
                </div>
            </div>
        </div>
    };

    create_portal(
        html! {
            {render_component}
        },
        modal_host.into(),
    )
}

fn get_single_ele_from_vec_by_default(
    map: &HashMap<&'static str, Vec<Html>>,
    key: &'static str,
) -> Html {
    let vec = map.get(key).unwrap();

    match vec.get(0) {
        Some(vnode) => vnode.clone(),
        None => html! {},
    }
}

fn extract_nodes<'a>(
    children: &ChildrenRenderer<ModalVariant>,
    map: &'a mut HashMap<&'static str, Vec<Html>>,
) {
    for child in children.iter() {
        match child.modal_component_props {
            ModalComponentProps::Action(_) => {
                append_child(child, "action", map);
            }
            ModalComponentProps::Content(_) => {
                append_child(child, "content", map);
            }
        }
    }
}

fn append_child<'a>(
    child: ModalVariant,
    key: &'static str,
    map: &'a mut HashMap<&'static str, Vec<Html>>,
) {
    let v = map.get_mut(key).unwrap();

    v.push(child.into());
}
