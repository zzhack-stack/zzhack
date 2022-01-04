use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct ProjectCardProps {
    pub title: String,
    pub desc: String,
    pub link: String,
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
            height: 150px;
            background: var(--base-color);
            border-radius: 5px;
            padding: 16px 20px 16px 20px;
            box-sizing: border-box;
            transition: all 0.3s;
            cursor: pointer;

            &:hover {
                background: var(--technology-hover-color);
            }

            .desc {
                font-size: 14px;
                color: var(--sub-text-color);
                line-height: 23px;
            }

            .title {
                font-size: 16px;
                font-weight: 500;
                color: var(--base-text-color);
                line-height: 22px;
            }

            @media (max-width: 600px) {
                width: 100%;
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
            <a style="text-decoration: none;" target="_blank" href=self.props.link.clone()>
                <div class=format!("{} card", self.style.to_string())>
                    <div class="title">{self.props.title.clone()}</div>
                    <div class="desc">{self.props.desc.clone()}</div>
                </div>
            </a>
        }
    }
}
