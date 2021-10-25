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

const CONTACTS: [&'static str; 5] = ["github", "twitter", "discord", "mail", "wechat"];

impl Component for Footer {
    type Message = FooterMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Footer",
            r#"
            width: 100%;
            box-sizing: border-box;
            padding-top: 33px;
            padding-bottom: 16px;

            .bee {

            }

            .contacts {
                margin-top: 14px;
            }

            .contact-icon {
                margin-right: 10px;
            }

            .footer-info {
                display: flex;
                justify-content: space-between;
            }

            .copyright {
                text-align: center;
                margin-top: 23px;
            }

            @media (max-width: 600px){
                flex-direction: column;

                .separator {
                    display: none;
                }

                .footer-info {
                    flex-direction: column;
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
            <div class="container">
                <div class=self.style.to_string()>
                    <div class="footer-info">
                        <div>
                            <div>
                                {"Powered by Rust & Yew"}
                            </div>
                            <div>
                                {"Illustration by Icons 8 from Ouch!"}
                            </div>
                        </div>
                        <div>
                            <div class="contacts">
                                {for CONTACTS.iter().map(|contact| {
                                    html! {
                                        <img class="contact-icon" src={format!("/images/{}_icon.svg", contact)} />
                                    }
                                })}
                            </div>
                        </div>
                    </div>
                    <div class="copyright">
                        <img src="/images/bee.svg" class="bee" />
                        {"Copyright © 2021 Mist"}
                    </div>
                </div>
            </div>
        }
    }
}
