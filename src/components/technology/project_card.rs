use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct ProjectCardProps {
    pub title: String,
    pub desc: String,
}

pub struct ProjectCard {
    style: Style,
    props: ProjectCardProps,
}

impl Component for ProjectCard {
    type Message = ();
    type Properties = ProjectCardProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "ProjectCard",
            r#"
            width: 300px;
            height 150px;
            background: var(--base-color);
            border-radius: 5px;
            border: 1px solid rgba(27, 31, 35, 0.15);
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
                <div class="title">{self.props.title.clone()}</div>
                <div class="desc">{self.props.desc.clone()}</div>
            </div>
        }
    }
}
