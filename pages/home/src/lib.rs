use services::post_service::POST_SERVICE;
use ui::image::ThemeImage;
use ui::post_card::post_card::PostCard;
use utils::use_style;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
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

        }

        .posts {
            margin-bottom: 45px;
            margin-top: 39px;
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
            }
        }
    "
    );

    html! {
        <div class={style}>
            <div class="banner">
                <ThemeImage source="banner.svg" is_reactive=true />
            </div>
            <div class="labels">
            </div>
            <div class="posts">
                {
                    POST_SERVICE.get_posts().iter().map(|post| {
                        html! {
                            <PostCard post={post.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
