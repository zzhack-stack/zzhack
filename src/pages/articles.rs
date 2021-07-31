use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct ArticleProps {
    pub number: u32,
}

pub struct Articles {
    props: ArticleProps,
    style: Style,
}

pub enum ArticlesMessage {}

impl Component for Articles {
    type Message = ArticlesMessage;
    type Properties = ArticleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Articles",
            r#"
        "#,
        )
        .unwrap();

        Self { props, style }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
            </div>
        }
    }
}
