use css_in_rust::Style;
use yew::prelude::*;

pub struct AboutMe {
    style: Style,
}

pub enum AboutMeMessage {}

impl Component for AboutMe {
    type Message = AboutMeMessage;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "AboutMe",
            r#"
            min-height: calc(100% - 48px);
        "#,
        )
        .unwrap();

        Self { style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                // <BookView number=2 />
            </div>
        }
    }
}
