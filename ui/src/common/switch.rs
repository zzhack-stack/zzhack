use global::theme_context::ThemeAction;
use global::theme_context::ThemeContext;
use services::post_service::PostService;
use services::posts::POSTS;
use services::theme_service::Theme;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(ThemeSwitchBar)]
pub fn theme_switch_bar() -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    log::info!("{}", &theme_ctx.theme.into_str());
    let mobile_sliding_bar_offset_pos = if &theme_ctx.theme == &Theme::Dark {
        "100%"
    } else {
        "0"
    };
    let pc_sliding_bar_offset_pos = if &theme_ctx.theme == &Theme::Dark {
        "-100%"
    } else {
        "-190%"
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
            transform: translateY(${pc_offset});
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

        @media (max-width: 600px) {
            width: 100%;
            height: 43px;
            display: flex;
            align-items: center;
            margin-top: 33px;

            .switch-bar {
                width: 50%;
                height: 100%;
            }

            .sliding-block {
                width: 50%;
                top: 0;
                height: 100%;
                transform: translateX(${mobile_offset});
            }
        }
    "#,
        pc_offset = pc_sliding_bar_offset_pos,
        mobile_offset = mobile_sliding_bar_offset_pos
    );

    let handle_switch_bar_click = |theme: Theme| -> Callback<MouseEvent> {
        Callback::from(move |_| theme_ctx.dispatch(ThemeAction::UpdateTheme(theme.clone())))
    };

    log::info!("{:?}", PostService::get_posts());

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
