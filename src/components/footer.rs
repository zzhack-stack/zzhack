use yew::prelude::*;
use css_in_rust::Style;

pub struct Footer {
    style: Style
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("Footer", r#"
            
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
                {"404"}
            </div>
        }
    }
}