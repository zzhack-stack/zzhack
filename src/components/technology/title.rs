use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct TechnologyTitleProps {
    pub title: String,
    pub icon: String,
}

pub struct TechnologyTitle {
    style: Style,
    props: TechnologyTitleProps,
}

impl Component for TechnologyTitle {
    type Message = ();
    type Properties = TechnologyTitleProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "TechnologyTitle",
            r#"
            display: flex;
            align-items: center;

            .icon {}

            .title {
                font-size: 25px;
                font-weight: 500;
                color: var(--base-text-color);
                line-height: 36px;
                margin-left: 10px;
            }
        "#,
        )
        .unwrap();

        Self { style, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <img class="icon" src={self.props.icon.clone()} />
                <span class="title">{self.props.title.clone()}</span>
            </div>
        }
    }
}
