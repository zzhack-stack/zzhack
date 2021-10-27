use crate::components::Banner;
use css_in_rust::Style;
use yew::prelude::*;

pub struct Thinking {
    style: Style,
}

impl Component for Thinking {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Thinking",
            r#"
            @media (max-width: 600px) {

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
                <Banner illustration_style="top: 15px;" bg_color="var(--thinking-banner-color)" illustration="/images/thinking_illustration.svg" />
            </div>
        }
    }
}
