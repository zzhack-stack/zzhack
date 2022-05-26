use crate::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ContainerProps {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    let style = use_style!(
        r"
        width: 100%;
        display: flex;
        justify-content: center;

        .container-box {
            max-width: 1020px;
            width: 65%;
        }

        @media (max-width: 600px) {
            .container-box {
                width: 100%;
                padding: 0 22px;
            }
        }
   "
    );

    html! {
        <div class={style}>
            <div class="container-box">
                {props.children.clone()}
            </div>
        </div>
    }
}
