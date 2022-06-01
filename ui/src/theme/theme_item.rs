use services::theme_service::Theme;
use stylist::style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ThemeItemProps {
    pub source: &'static str,
    pub text: &'static str,
    pub theme: Theme,
    pub is_pick: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ThemeItem)]
pub fn theme_item(props: &ThemeItemProps) -> Html {
    let picked_calculator = |picked_value: &'static str| -> &'static str {
        if props.is_pick {
            picked_value
        } else {
            "0"
        }
    };

    let picked_border_radius = picked_calculator("10px");
    let picked_border_width = picked_calculator("2.5px");
    let picked_shadow = picked_calculator("var(--primary-shadow-color) 0px 8px 24px");
    let style = style!(
        r"
        display: flex;
        flex-direction: column;
        align-items: center;

        .theme-preview {
            width: 100px;
            transition: all 0.1s;
            border: 2.5px solid #7FD6C2;
            border-radius: 10px;
            box-sizing: border-box;
            border: 2.5px solid var(--primary-color);
            border-width: ${border_width};
            border-radius: ${border_radius};
            box-shadow: ${shadow};
            cursor: pointer;
        }

        .text {
            color: var(--text-color);
            font-size: 14px;
            margin-top: 8px;
            font-weight: bold;
        }

        @media (max-width: 600px) {
            margin: 5px 0;

            .text {
                font-size: 12px;
            }
        }
    ",
        border_radius = picked_border_radius,
        border_width = picked_border_width,
        shadow = picked_shadow
    )
    .unwrap();

    html! {
        <div class={style} onclick={props.onclick.clone()}>
            <img class="theme-preview" src={props.source} />
            <span class="text">{props.text}</span>
        </div>
    }
}
