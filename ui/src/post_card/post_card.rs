use crate::post_card_large::PostCardLarge;
use crate::post_card_small::PostCardSmall;
use services::post_service::post_card_size::PostCardSize;
use services::post_service::post_service::Post;
use utils::theme::is_on_mobile;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct PostCardProps {
    pub post: Post,
    #[prop_or(PostCardSize::Large)]
    pub size: PostCardSize,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    if is_on_mobile() {
        return html! {
            <PostCardSmall post={props.post.clone()} />
        };
    }

    match &props.size {
        PostCardSize::Large => html! {
            <PostCardLarge post={props.post.clone()} />
        },
        PostCardSize::Small => html! {
            <PostCardSmall post={props.post.clone()} />
        },
    }
}
