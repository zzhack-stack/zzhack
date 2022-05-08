use material_yew::MatButton;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalActionProps {
    pub label: &'static str,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ModalAction)]
pub fn modal_action(props: &ModalActionProps) -> Html {
    let style = use_style!(
        r"
        margin: 0 10px;
    "
    );
    html! {
        <div class={style} onclick={props.onclick.clone()}>
            <MatButton label={props.label} />
        </div>
    }
}
