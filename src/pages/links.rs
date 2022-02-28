use crate::provider_service;
use crate::services::provider_service::Links as LinksData;
use crate::Footer;
use css_in_rust::Style;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

#[derive(Properties, Clone)]
pub struct LinksProps {}

pub struct Links {
    pub style: Style,
    pub props: LinksProps,
    link: ComponentLink<Links>,
    fetch_links_task: Option<FetchTask>,
    links_data: LinksData,
}

pub enum LinksMessage {
    UpdateLinksData(LinksData),
}

impl Component for Links {
    type Message = LinksMessage;
    type Properties = LinksProps;
    fn create(props: LinksProps, link: ComponentLink<Links>) -> Self {
        let style = Style::create(
            "Links",
            r#"
            .link__banner {
                height: 400px;
                width: 100%;    
                background: var(--links-banner-color);
                display: flex;
                align-items: center;
            }

            .links-wrapper {
                min-height: calc(100vh - 233.39px);
            }

            .link__banner__content {
                display: flex;
            }

            .link__banner__img {
                width: 500px;
                position: relative;
                right: -100px;
            }

            .link__title {
                font-size: 30px;
                font-weight: 500;
                line-height: 42px;
                margin-bottom: 10px;
            }

            .link__request-button {
                width: 150px;
                height: 50px;
                background: white;
                border-radius: 10px;
                color: var(--banner-text-color);
                display: flex;
                justify-content: center;
                align-items: center;
                box-shadow: rgba(0, 0, 0, 0.15) 1.95px 1.95px 2.6px;
                transition: box-shadow 0.3s;
                margin-top: 30px;
            }

            .link__request-button:hover {
                box-shadow: rgba(50, 50, 93, 0.25) 0px 30px 60px -12px, rgba(0, 0, 0, 0.3) 0px 18px 36px -18px;
            }
            
            .links-data {
                display: flex;
                flex-wrap: wrap;
                margin: 0 -20px;
                margin-top: 50px;
            }

            .link-item {
                background: var(--base-color);
                box-shadow: rgb(50 50 93 / 25%) 0px 2px 5px -1px, rgb(0 0 0 / 30%) 0px 1px 3px -1px;
                border-radius: 5px;
                margin: 0 20px;
                padding: 18px 25px;
                display: flex;
                align-items: center;
                transition: box-shadow 0.3s;
            }

            .link-item:hover {
                box-shadow: rgba(50, 50, 93, 0.25) 0px 30px 60px -12px, rgba(0, 0, 0, 0.3) 0px 18px 36px -18px;
            }

            .link-item__logo {
                width: 45px;
                height: 45px;
                border-radius: 50%;
            }

            .link-item__info {
                margin-left: 15px;
            }

            .link-item__info__desc {
                font-size: 12px;
            }

            @media (max-width: 600px) {
                .link__banner {
                    height: auto;
                    padding: 50px 0;
                }

                .link__banner__img {
                    display: none;
                }

                .links-data {
                    padding: 50px 0;
                    margin: auto;
                }

                .link-item {
                    margin: auto;
                    margin-bottom: 20px;
                }
                
                .link-item__link {
                    width: 100%;
                }
            }
        "#,
        )
        .unwrap();
        Self {
            style,
            props,
            link,
            fetch_links_task: None,
            links_data: vec![],
        }
    }

    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            LinksMessage::UpdateLinksData(links_data) => {
                self.links_data = links_data;
                true
            }
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn rendered(&mut self, is_first_render: bool) {
        if is_first_render {
            self.fetch_links_task = Some(
                provider_service.fetch_links(
                    self.link
                        .callback(|links| LinksMessage::UpdateLinksData(links)),
                ),
            );
        }
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        html! {
            <div class=self.style.to_string()>
                <div class="links-wrapper">
                    <div class="link__banner">
                        <div class="link__banner__content container">
                            <div class="link__banner__left">
                                <div>
                                    <div class="link__title">{"Links"}</div>
                                    <div>{"The links can help make better SEO and external link, this page is used to display links and you can send me any links you want to display by click the follow button if you want"}</div>
                                </div>
                                <a href="https://github.com/zzhack-stack/zzhack/issues/4" class="non-style-link">
                                    <div class="link__request-button">
                                        {"Request"}
                                    </div>
                                </a>
                            </div>
                            <img class="link__banner__img" src="/images/link_banner_illustration.svg" />
                        </div>
                    </div>
                    <div class="container">
                        <div class="links-data">
                            {
                                for self.links_data.iter().map(|data| html! {
                                    <a class="link-item__link non-style-link" href=data.link.to_string()>
                                        <div class="link-item">
                                            <img class="link-item__logo" src=data.logo.to_string() />
                                            <div class="link-item__info">
                                                <div class="link-item__info__name">
                                                    {&data.name}
                                                </div>
                                                <div class="link-item__info__desc">
                                                    {&data.desc}
                                                </div>
                                            </div>
                                        </div>
                                    </a>
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
