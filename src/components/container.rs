use yew::prelude::*;
use css_in_rust::Style;

#[derive(Properties, Clone)]
pub struct ContainerProps {
    pub children: Children
}

pub struct Container {
    style: Style,
    props: ContainerProps
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("Footer", r#"
            width: 1228px;
            margin: auto;
        "#).unwrap();

        Self {
            props,
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
                {self.props.children.clone()}
            </div>
        }
    }
}