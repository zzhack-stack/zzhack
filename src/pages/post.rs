use crate::components::comments::Comments;
use crate::components::Banner;
use crate::services::post_service::PostService;
use crate::services::provider_service::PostMetadata;
use crate::services::MarkdownService;
use crate::utils::theme::by_theme;
use crate::utils::time::format_timestamp;
use crate::Footer;
use css_in_rust::Style;
use material_yew::MatCircularProgressFourColor;
use web_sys::Element;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct PostProps {
    pub filename: String,
    pub category: String,
}

pub struct Post {
    style: Style,
    link: ComponentLink<Post>,
    props: PostProps,
    post_metadata: Option<PostMetadata>,
    parsed_content: Option<Element>,
}

pub enum PostMessage {
    UpdatePostMetadata(PostMetadata),
    ParseContentToElement,
}

impl Component for Post {
    type Message = PostMessage;
    type Properties = PostProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Post",
            r#"
            background: var(--post-undercover-color);

            .post-container {
                box-sizing: border-box;
                padding: 20px;
                background: var(--base-color);
                border-radius: 5px;
                margin: 30px auto 0 auto;
            }

            .posts-loading {
                width: 100%;
                display: flex;
                justify-content: center;
                padding-top: 100px;
                box-sizing: border-box;
                min-height: calc(100vh - 475px);
            }

            .post-cover {
                width: 100%;
                height: 360px;
                border-radius: 6px;
                background-repeat: no-repeat;
                background-position: 50%;
                background-size: cover;
            }

            .post-title {
                font-size: 40px;
                margin-top: 44px;
            }

            .post-info {
                font-size: 16px;
                color: var(--sub-text-color);
                opacity: 0.48;
            }

            .post-date {
                margin-right: 28px;
            }

            @media (max-width: 600px) {
                .post-cover {
                    height: 200px;
                }

                .post-title {
                    font-size: 32px;
                }
            }
        "#,
        )
        .unwrap();

        Self {
            style,
            props,
            link,
            post_metadata: None,
            parsed_content: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PostMessage::UpdatePostMetadata(metadata) => {
                self.post_metadata = Some(metadata);
                self.link.send_message(PostMessage::ParseContentToElement);

                false
            }
            PostMessage::ParseContentToElement => {
                let post_metadata = self.post_metadata.clone().unwrap();
                let markdown_service = MarkdownService::new(post_metadata.content);

                self.parsed_content = Some(
                    markdown_service
                        .parse_to_element(by_theme("base16-ocean.light", "base16-ocean.dark")),
                );
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

        let post_service = PostService::new();
        let post_metadata =
            post_service.get_post_metadata(&self.props.category, &self.props.filename);

        self.link
            .send_message(PostMessage::UpdatePostMetadata(post_metadata))
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <Banner bg_color="var(--post-banner-color)" illustration="/images/post_illustration.svg" illustration_style="top: 31px;right: -150px;" />
                {
                    if self.parsed_content.is_none() {
                        html! {
                            <div class="posts-loading">
                                <MatCircularProgressFourColor indeterminate=true />
                            </div>
                        }
                    } else {
                        let parsed_metadata = self.post_metadata.clone().unwrap();

                        html! {
                            <div class="post-container mini-container">
                                <div class="post-cover" style=format!("background-image: url({});", parsed_metadata.cover)></div>
                                <div class="post-title">{parsed_metadata.title}</div>
                                <div class="post-info">
                                    <span class="post-date">{format_timestamp(parsed_metadata.create_at, "%Y/%m/%d")}</span>
                                    <span>{"Mist"}</span>
                                </div>

                                {Html::VRef(self.parsed_content.clone().unwrap().into())}
                            </div>
                        }
                    }
                }
                {
                    if let Some(post_metadata) = &self.post_metadata  {
                        html! {
                            <Comments issue_number={post_metadata.issue_id.clone()} />
                        }
                    } else {
                        html! {<></>}
                    }
                }
                <Footer />
            </div>
        }
    }
}
