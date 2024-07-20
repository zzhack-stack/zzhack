use shared::post::Post;
use yew::prelude::*;
use yew_router::components::Link;

use crate::routes::Routes;

#[derive(Properties, PartialEq)]
pub struct PostItemProps {
    pub post: Post,
}

#[function_component]
pub fn PostItem(props: &PostItemProps) -> Html {
    let post = &props.post;

    html! {
        <a href={format!("/post/{}", post.id)}>
            <div>
                <h2>{&post.title}</h2>
                <p>{&post.spoiler}</p>
                <p>{&post.created_at}</p>
            </div>
        </a>
    }
}
