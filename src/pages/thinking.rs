use crate::components::common::post_card::PostCard;
use crate::components::Banner;
use crate::services::provider_service::PostMetadata;
use crate::AppRouterAnchor;
use crate::AppRoutes;
use crate::CacheService;
use crate::Footer;
use css_in_rust::Style;
use yew::prelude::*;

pub struct Thinking {
    style: Style,
    thinking_posts: Vec<PostMetadata>,
}

impl Component for Thinking {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Thinking",
            r#"
            
            .thinking-posts {
                margin: 60px -30px;
                display: flex;
            }

            .thinking-post__goto {
                text-decoration: none;
            }

            @media (max-width: 600px) {
                .thinking-posts {
                    flex-direction: column;
                    margin: 60px 0;
                }
            }
        "#,
        )
        .unwrap();
        let root_metadata = CacheService::new().get_root_metadata();
        let thinking_posts = root_metadata.categories.thinking;

        Self {
            style,
            thinking_posts,
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
                <Banner illustration_style="top: 15px;right: -145px;" bg_color="var(--thinking-banner-color)" illustration="/images/thinking_illustration.svg" />
                <div class="container">
                    <div class="thinking-posts">
                        {
                            for self.thinking_posts.clone().iter().map(|metadata| {
                                html! {
                                    <AppRouterAnchor classes="thinking-post__goto" route={AppRoutes::Post(String::from("thinking"), metadata.filename.clone())}>
                                        <PostCard cover=metadata.cover.clone() title=metadata.title.clone() summary=metadata.summary.clone() create_at=metadata.create_at />
                                    </AppRouterAnchor>
                                }
                            })
                        }
                    </div>
                </div>
                <Footer />
            </div>
        }
    }
}
