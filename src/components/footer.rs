use crate::utils::theme::by_theme;
use css_in_rust::Style;
use material_yew::MatIconButton;
use yew::prelude::*;

pub struct Footer {
    style: Style,
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Footer",
            r#"
            width: 100%;
            padding: 10px 0;
            background: #636e72;
            display: flex;
            justify-content: center;
            align-items: center;

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
                color: white;
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

        Self { style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="powered-by">
                    <span class="footer-text">
                        {"Powered by"}
                    </span>
                    <a href="https://www.rust-lang.org/">
                        <MatIconButton>
                            <img class="icon" src="/images/rust_icon.svg" />
                        </MatIconButton>
                    </a>
                </div>
                <div class="separator" />
                <span class="footer-text">
                    {"Copyright © 2021 ZhaoZhanHao"}
                </span>
            </div>
        }
    }
}
