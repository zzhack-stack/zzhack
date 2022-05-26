use crate::theme::theme_item::ThemeItem;
use global::theme_context::ThemeAction;
use global::theme_context::ThemeContext;
use services::theme_service::Theme;
use services::theme_service::ThemeService;
use stylist::style;
use yew::prelude::*;

struct ThemeItemData {
    source: &'static str,
    text: &'static str,
    theme: Theme,
}

static THEME_ITEMS: &[ThemeItemData; 3] = &[
    ThemeItemData {
        source: "images/light_mode_skeleton.png",
        text: "Light",
        theme: Theme::Light,
    },
    ThemeItemData {
        source: "images/dark_mode_skeleton.png",
        text: "Dark",
        theme: Theme::Dark,
    },
    ThemeItemData {
        source: "images/auto_mode_skeleton.png",
        text: "Follow OS",
        theme: Theme::Auto,
    },
];

#[function_component(ThemeSelector)]
pub fn theme_selector() -> Html {
    let current_theme = use_state_eq(|| ThemeService::get_theme_from_storage());
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let style = style!(
        r"
        .skeletons {
            width: 360px;
            display: flex;
            justify-content: space-between;
        }

        .skeleton {
            width: 100px;
        }

        .follow-os-theme-wrapper {
            display: flex;
            align-items: center;
        }

        @media (max-width: 600px) {
            .skeletons {
                width: 100%;
                display: flex;
                flex-direction: column;
            }
        }
    "
    )
    .unwrap();
    let handle_theme_item_click = |theme: &Theme| -> Callback<MouseEvent> {
        let theme = theme.clone();
        let current_theme = current_theme.clone();

        Callback::from(move |_| {
            current_theme.set(theme.clone());
            theme_ctx.dispatch(ThemeAction::UpdateTheme(theme.clone()));
        })
    };

    html! {
        <div class={style}>
            <div class="skeletons">
                {THEME_ITEMS.into_iter().map(|data| {
                    html! {
                        <ThemeItem source={data.source} text={data.text} theme={data.theme.clone()} is_pick={*current_theme == data.theme} onclick={handle_theme_item_click.clone()(&data.theme)} />
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
