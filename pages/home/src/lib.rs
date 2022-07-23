use services::post_service::post_service::{FilterTag, PostService};
use ui::image::ThemeImage;
use ui::label::Label;
use ui::link::Link;
use ui::post_card::post_card::PostCard;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub posts_key: String,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let style = use_style!(
        r"
        .banner {
            width: 100%;
            display: flex;
            justify-content: center;
            margin-top: 40px;
        }

        .banner > img {
            width: 100%;
        }

        .labels {
            width: 100%;
            display: flex;
            flex-wrap: wrap;
            margin-top: 33px;
        }

        .label {
            margin-left: 18px;
        }

        .posts {
            margin-bottom: 45px;
            margin-top: 39px;
            display: flex;
            flex-wrap: wrap;
            margin: 30px -18px 45px -18px;
        }

        @media (max-width: 600px) {
            .banner {
                width: 100%;
                margin-top: 32px;
            }

            .posts {
                display: flex;
                flex-direction: column;
                align-items: center;
                margin: 39px 0 45px 0;
            }
        }
    "
    );
    let post_service = PostService::from(props.posts_key.clone());
    let posts = use_state_eq(|| post_service.get_posts());
    let handle_filter_posts_by_label = {
        let posts = posts.clone();
        let post_service = post_service.clone();

        |tag: FilterTag| {
            Callback::from(move |_| {
                posts.set(post_service.filter_post_by_tag(tag.clone()));
            })
        }
    };
    let handle_filter_posts_by_rest_label = handle_filter_posts_by_label.clone();
    let tags = post_service.get_tags().clone();

    html! {
        <div class={style}>
            <div class="banner">
                <ThemeImage source="banner.svg" is_reactive=true />
            </div>
            <div class="labels">
                <Link onclick={handle_filter_posts_by_label(FilterTag::All)}>
                    <Label text="All" />
                </Link>
                {tags.iter().map(|tag| {
                    html! {
                        <div class="label">
                            <Link onclick={handle_filter_posts_by_rest_label.clone()(FilterTag::Tag(tag.clone()))}>
                                <Label text={tag.clone()} />
                            </Link>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
            <div class="posts">
                {
                    posts.iter().map(|post| {
                        html! {
                            <PostCard size={post.metadata.size.clone()} post={post.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
