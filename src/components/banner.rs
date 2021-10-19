use crate::workers::theme_agent::ThemeAgent;
use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct BannerProps {
    pub bg_color: String,
    pub illustration: String,
}

pub struct Banner {
    style: Style,
    props: BannerProps,
    theme_agent: Box<dyn yew::Bridge<ThemeAgent>>,
}

pub enum BannerMessage {
    ChangeTheme,
}

const CONTACTS: [&'static str; 5] = ["github", "twitter", "discord", "mail", "wechat"];

impl Component for Banner {
    type Message = BannerMessage;
    type Properties = BannerProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Banner",
            r#"
            width: 100%;
            height: 244px;

            .wrapper {
                position: relative;
            }

            .greeter {
                padding-top: 31px;
                display: flex;
            }

            .greeter-text {
                margin-right: 13px;
                font-size: 30px;
                font-weight: 500;
                color: var(--base-text-color);
                line-height: 42px;
            }

            .desc {
                margin-top: 14px;
                font-size: 14px;
                font-weight: 300;
                color: var(--sub-text-color);
                line-height: 20px;
                width: 729px;
            }

            .contact-title {
                font-size: 18px;
                font-weight: 500;
                color: #3F3D55;
                line-height: 25px;
                padding-top: 14px;
            }

            .contacts {
                margin-top: 14px;
            }

            .contact-icon {
                margin-right: 10px;
            }

            .illustration {
                position: absolute;
                right: 0;
                top: 61px;
                right: -50px;
            }


            @media (max-width: 600px) {
                .illustration {
                    display: none;
                }
                
                .desc {
                    width: 100%;
                }
            }
        "#,
        )
        .unwrap();
        let theme_agent = ThemeAgent::bridge(link.callback(|_| BannerMessage::ChangeTheme));

        Self {
            style,
            props,
            theme_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BannerMessage::ChangeTheme => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string() style=format!("background: {};", self.props.bg_color)>
                <div class="container wrapper">
                    <div class="greeter">
                        <div class="greeter-text">
                            {"I'm Mist"}
                        </div>
                        <img src="/images/avatar.svg" />
                    </div>
                    <div class="desc">
                        {"<Web /> & Block chain & Native developer, Passionate C++, Python, TypeScript, Golang, Rust developer, like math also Haskell. Remote worker."}
                    </div>
                    <div class="contact-wrapper">
                        <div class="contact-title">
                            {"Contact with MI"}
                        </div>
                        <div class="contacts">
                            {for CONTACTS.iter().map(|contact| {
                                html! {
                                    <img class="contact-icon" src={format!("/images/{}_icon.svg", contact)} />
                                }
                            })}
                        </div>
                    </div>
                    <img class="illustration" src={self.props.illustration.clone()} />
                </div>
            </div>
        }
    }
}
