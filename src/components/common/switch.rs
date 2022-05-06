use crate::global::theme_context::Theme;
use crate::global::theme_context::ThemeAction;
use crate::global::theme_context::ThemeContext;
use crate::use_style;
use crate::utils::theme::by_reactive;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ThemeSwitchProps {
    pub default: Theme,
}

#[styled_component(ThemeSwitchPC)]
pub fn theme_switch_pc(props: &ThemeSwitchProps) -> Html {
    let ctx = use_context::<ThemeContext>().unwrap();
    let sliding_bar_top_pos = if &ctx.theme == &Theme::Dark {
        "50%"
    } else {
        "0"
    };
    let style = css!(
        r#"
        width: 54px;
        height: 162px;
        background: var(--primary-color);
        box-shadow: 0px 8px 24px 0px rgba(149, 157, 165, 0.5);
        border-radius: 16px;
        position: relative;

        .sliding-block {
            position: absolute;
            top: ${top};
            left: 0;
            width: 54px;
            height: 86px;
            border-radius: 16px;
            background: #fff;
            transition: all 0.3s ease-out;
        }

        .switch-bar {
            position: relative;
            z-index: 1;
            height: 50%;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
        }
    "#,
        top = sliding_bar_top_pos
    );

    let handle_switch_bar_click = |theme: Theme| -> Callback<MouseEvent> {
        Callback::from(move |_| ctx.dispatch(ThemeAction::UpdateTheme(theme.clone())))
    };

    html! {
        <div class={style}>
            <div onclick={handle_switch_bar_click.clone()(Theme::Light)} class="light-bar switch-bar">
                <img src="/images/light_mode.svg" />
            </div>
            <div onclick={handle_switch_bar_click(Theme::Dark)} class="dark-bar switch-bar">
                <img src="/images/dark_mode.svg" />
            </div>
            <div class="sliding-block" />
        </div>
    }
}

#[function_component(ThemeSwitchMobile)]
pub fn theme_switch_mobile(props: &ThemeSwitchProps) -> Html {
    let style = use_style!(r#""#);

    html! {
        <div style={style}>
        </div>
    }
}

#[function_component(ThemeSwitch)]
pub fn theme_switch(props: &ThemeSwitchProps) -> Html {
    let style = use_style!(r#""#);

    by_reactive(
        html! {<ThemeSwitchMobile default={props.default.clone()} />},
        html! {<ThemeSwitchPC default={props.default.clone()} />},
    )
}
