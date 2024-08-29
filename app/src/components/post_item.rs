use shared::post::Post;
use yew::prelude::*;

use crate::components::tag::Tag;

#[derive(Properties, PartialEq)]
pub struct PostItemProps {
    pub post: Post,
}

#[function_component]
pub fn PostItem(props: &PostItemProps) -> Html {
    let post = &props.post;
    let tags = post
        .tags
        .iter()
        .map(|tag| {
            html! {
                <Tag>{tag.text.clone()}</Tag>
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <a href={format!("/post/{}", post.id)}>
            <div class="my-5">
                <div class="text-xl font-bold text-black">
                    {&post.title}
                    <div>{tags}</div>
                </div>
                <div class="text-black-700 text-sm mt-0.5 mb-1">{&post.spoiler}</div>
                <div class="text-xs text-black-500">{&post.created_at}</div>
            </div>
        </a>
    }
}
