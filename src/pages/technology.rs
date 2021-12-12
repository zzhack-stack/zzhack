use crate::components::technology::post_card::PostCard;
use crate::components::technology::project_card::ProjectCard;
use crate::components::{technology::title::TechnologyTitle, Banner};
use crate::services::provider_service::RootMetadata;
use crate::services::provider_service::{provider_service, PinnedProject, PinnedProjects};
use crate::CacheService;
use crate::Footer;
use css_in_rust::Style;
use material_yew::MatCircularProgressFourColor;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct Technology {
    style: Style,
    projects_fetch_task: FetchTask,
    pinned_projects: Vec<PinnedProject>,
    root_metadata: RootMetadata,
}

pub enum TechnologyMessage {
    UpdatePinnedProjects(PinnedProjects),
}

const GITHUB_HOMEPAGE: &'static str = "https://github.com/mistricky";

impl Component for Technology {
    type Message = TechnologyMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Technology",
            r#"
            min-height: calc(100% - 48px);

            .projects {
                margin-top: 43px;
                width: 960px;
                height: 386px;
            }

            .project-cards {
                display: flex;
                flex-wrap: wrap;
                justify-content: space-between;
            }

            .project-card-wrapper {
                margin-top: 25px;
            }

            .open-source {
                margin-top: 50px;
                display: flex;
            }

            .open-source__title {
                font-size: 30px;
                font-weight: 500;
                color: var(--base-text-color);
                line-height: 42px;
                margin-top: 17px;
            }

            .loading {
                width: 100%;
                height: 100%;
                display: flex;
                justify-content: center;
                align-items: center;
            }

            .open-source__goto {
                width: 159px;
                height: 41px;
                background: var(--base-color);
                border-radius: 5px;
                border: 1px solid #979797;
                display: flex;
                justify-content: center;
                align-items: center;
                cursor: pointer;
                transition: all 0.3s;
                margin-top: 20px;
            }

            .open-source__goto:hover {
                background: var(--technology-hover-color);
            }

            .posts {
                margin-top: 42px;
            }

            .posts__collection {
                width: 100%;
                padding: 33px 36px;
                box-sizing: border-box;
                background: var(--base-color);
                border-radius: 5px;
                border: 1px solid var(--border-color);
                margin-top: 25px;
            }

            .github_btn {
                text-decoration: none;
            }

            .post-loading {
                width: 100%;
                height:100%;
                text-align: center;
            }

            @media (max-width: 600px) {
                .project-card-wrapper {
                    width: 100%;
                }

                .projects {
                    height: auto;
                    width: auto;
                }

                .open-source__illustration {
                    display: none;
                }

                .posts__collection {
                    padding: 21px 22px;
                }
            }
        "#,
        )
        .unwrap();
        let projects_fetch_task =
            provider_service.get_pinned_projects(link.callback(|pinned_projects| {
                TechnologyMessage::UpdatePinnedProjects(pinned_projects)
            }));
        let root_metadata = CacheService::new().get_root_metadata();

        Self {
            style,
            projects_fetch_task,
            pinned_projects: vec![],
            root_metadata,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TechnologyMessage::UpdatePinnedProjects(pinned_projects) => {
                self.pinned_projects = pinned_projects.projects;
                true
            }
            _ => false,
        }
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
                            {
                                if self.pinned_projects.len() == 0 {
                                    html! {
                                        <div class="loading">
                                            <MatCircularProgressFourColor indeterminate=true />
                                        </div>
                                    }
                                } else {
                                    html! {
                                        for self.pinned_projects.iter().map(|project| {
                                            html! {
                                                <div class="project-card-wrapper">
                                                    <ProjectCard link=project.link.clone() title=project.title.clone() desc=project.desc.clone() />
                                                </div>
                                            }
                                        })
                                    }
                                }
                            }
                        </div>
                    </div>
                    <div class="open-source">
                        <div class="open-source__info">
                            <div class="open-source__title">
                                {"Open source"}
                            </div>
                            <div class="open-source__desc">
                                {"I will maintain some open source projects in my spare time, you can find more in my GitHub."}
                            </div>
                            <div class="open-source__goto">
                                <a class="github_btn" target="_blank" href=GITHUB_HOMEPAGE>
                                    {"> start GitHub"}
                                </a>
                            </div>
                        </div>
                        <img class="open-source__illustration" src="/images/open-source.svg" />
                    </div>
                    <div class="posts">
                        <TechnologyTitle title="Posts" icon="/images/cake.svg" />
                        <div class="posts__collection">
                            {
                                for self.root_metadata.clone().categories.technology.iter().map(|post_metadata| {
                                    html!{
                                        <PostCard post_metadata=post_metadata.clone() category="technology" />
                                    }
                                })
                            }
                        </div>
                    </div>
                </div>
                <Footer />
            </div>
        }
    }
}
