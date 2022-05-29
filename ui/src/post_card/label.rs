use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LabelProps {
    pub text: &'static str,
}

#[function_component(Label)]
pub fn label(props: &LabelProps) -> Html {
    let style = use_style!(
        r"
        padding: 8px 11px;
        background: var(--label-color);
        height: 39px;
        border-radius: 8px;
        box-sizing: border-box;
        display: flex;
        align-items: center;

        .label-icon {
            width: 17px;
            height: 17px;
            margin-right: 9px;
        }

        .label-text {
            font-size: 16px;
            color: var(--text-color);
        }
    "
    );

    html! {
        <div class={style}>
            <img class="label-icon" src="images/label.png" />
            <span class="label-text">
                {props.text}
            </span>
        </div>
    }
}
