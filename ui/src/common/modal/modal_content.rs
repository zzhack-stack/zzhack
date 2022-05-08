use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalContentProps {
    pub children: Children,
}

#[function_component(ModalContent)]
pub fn modal_content(props: &ModalContentProps) -> Html {
    html! {
        <>
            {props.children.clone()}
        </>
    }
}
