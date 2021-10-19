use crate::components::{
    technology::{project_card::ProjectCard, title::TechnologyTitle},
    Banner,
};
use css_in_rust::Style;
use yew::prelude::*;

pub struct Technology {
    style: Style,
}

impl Component for Technology {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Technology",
            r#"
            min-height: calc(100% - 48px);

            .projects {
                margin-top: 43px;
            }

            .project-cards {

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
                <Banner bg_color="var(--technology-banner-color)" illustration="/images/technology_illustration.svg"></Banner>
                <div class="container">
                    <div class="projects">
                        <TechnologyTitle title="Projects" icon="/images/projects_icon.svg" />
                        <div class="project-cards">
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
