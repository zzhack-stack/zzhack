use crate::utils::theme::by_theme;
use yew::prelude::*;
use css_in_rust::Style;
use material_yew:: {
    MatIconButton
};

pub struct Footer {
    style: Style
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("Footer", r#"
            width: 100%;
            padding: 10px 0;
            background: var(--base-color);
            display: flex;
            justify-content: center;
            align-items: center;

            .icon {
                width: 30px;
            }

            .separator {
                width: 1.5px;
                height: 20px;
                background: #636e72;
                margin-right: 10px;
            }
        "#).unwrap();

        Self {
            style
        }
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
                <span class="text">
                    {"Powered by"}
                </span>
                <a href="https://www.rust-lang.org/">
                    <MatIconButton>
                        <img class="icon" src="/images/rust_icon.svg" />
                    </MatIconButton>
                </a>
                <div class="separator" />
                <span class="text">
                    {"Copyright © 2021 ZhaoZhanHao"}
                </span>
            </div>
        }
    }
}