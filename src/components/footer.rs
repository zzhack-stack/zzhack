use crate::console_log;
use crate::utils::theme::by_theme;
use crate::workers::theme_agent::ThemeAgent;
use css_in_rust::Style;
use material_yew::MatIconButton;
use yew::prelude::*;

pub struct Footer {
    style: Style,
    theme_agent: Box<dyn yew::Bridge<ThemeAgent>>,
}

pub enum FooterMessage {
    ChangeTheme,
}

impl Component for Footer {
    type Message = FooterMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Footer",
            r#"
            width: 100%;
            padding: 10px 0;
            display: flex;
            justify-content: center;
            align-items: center;
            background: var(--base-color);

            .icon {
                width: 30px;
            }

            .separator {
                width: 1px;
                height: 18px;
                background: #636e72;
                margin-right: 10px;
            }

            .powered-by {
                display: flex;
                align-items: center;
            }

            .footer-text {
            }

            @media (max-width: 600px){
                flex-direction: column;

                .separator {
                    display: none;
                }
            }
        "#,
        )
        .unwrap();
        let theme_agent = ThemeAgent::bridge(link.callback(|_| FooterMessage::ChangeTheme));

        Self { style, theme_agent }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FooterMessage::ChangeTheme => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="powered-by">
                    <span class="text footer-text">
                        {"Powered by"}
                    </span>
                    <a href="https://www.rust-lang.org/">
                        <MatIconButton>
                            <img class="icon" src=by_theme("/images/rust_icon_dark.svg", "/images/rust_icon_light.svg") />
                        </MatIconButton>
                    </a>
                </div>
                <div class="separator" />
                <span class="text footer-text">
                    {"Copyright © 2021 ZhaoZhanHao"}
                </span>
            </div>
        }
    }
}
