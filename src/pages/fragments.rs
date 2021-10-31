use crate::components::Banner;
use crate::services::provider_service::provider_service;
use crate::services::provider_service::Fragments as FragmentsData;
use crate::Footer;
use css_in_rust::Style;
use material_yew::MatCircularProgressFourColor;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

pub struct Fragments {
    style: Style,
    fragments_fetch_task: Option<FetchTask>,
    fragments_data: Option<FragmentsData>,
    link: ComponentLink<Fragments>,
}

pub enum FragmentsMessage {
    UpdateFragmentsData(FragmentsData),
}

impl Component for Fragments {
    type Message = FragmentsMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Fragments",
            r#"

            .posts-loading {
                width: 100%;
                display: flex;
                justify-content: center;
                padding-top: 100px;
                box-sizing: border-box;
            }

            .fragments-wrapper {
                width: 100%;
                min-height: calc(100vh - 475px);
                padding-bottom: 150px;
                box-sizing: border-box;
            }

            .fragment-card {
                width: 600px;
                background: var(--base-color);
                border-radius: 6px;
                margin-top: 100px;
                box-shadow: rgb(17 12 46 / 15%) 0px 48px 100px 0px;
            }

            .fragment-cover {
                width: 600px;
                height: 270px;
                background-position: 50%;
                background-repeat: no-repeat;
                background-size: cover;
                border-top-left-radius: 6px;
                border-top-right-radius: 6px;
            }

            .fragment-card-wrapper {
                display: flex;
                justify-content: center;
            }

            .fragment-content {
                padding: 38px 28px;
            }

            .fragment-time {
                text-align: center;
                margin-bottom: 15px;
            }

            @media (max-width: 600px) {
                .fragment-card {
                    width: 100%;
                    margin-top: 60px;
                }

                .fragment-cover {
                    width: 100%;
                }
            }
        "#,
        )
        .unwrap();

        Self {
            style,
            link,
            fragments_fetch_task: None,
            fragments_data: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FragmentsMessage::UpdateFragmentsData(fragment_data) => {
                self.fragments_data = Some(fragment_data);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        self.fragments_fetch_task =
            Some(provider_service.get_fragments(
                self.link.callback(|fragments_data| {
                    FragmentsMessage::UpdateFragmentsData(fragments_data)
                }),
            ));
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <Banner bg_color="#EEBED3" illustration="/images/fragments_illustration.svg" illustration_style="top: 15px; right: -135px;" />
                <div class="fragments-wrapper container">
                    {
                        if self.fragments_data.is_none() {
                            html! {
                                <div class="posts-loading">
                                    <MatCircularProgressFourColor indeterminate=true />
                                </div>
                            }
                        } else {
                            html! {
                                for self.fragments_data.clone().unwrap().fragments.iter().map(|fragment| {
                                    html! {
                                        <div class="fragment-card-wrapper">
                                            <div class="fragment-card">
                                                <div class="fragment-cover" style=format!("background-image: url({});", fragment.cover)></div>
                                                <div class="fragment-content">
                                                    {&fragment.content}
                                                </div>
                                                <div class="fragment-time">{"2021/10/31"}</div>
                                            </div>
                                        </div>
                                    }
                                })
                            }
                        }
                    }
                </div>
                <Footer />
            </div>
        }
    }
}
