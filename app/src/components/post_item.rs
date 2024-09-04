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
                <Tag color={"#ff00ff"}>{tag.text.clone()}</Tag>
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <a href={format!("/post/{}", post.id)}>
            <div class="mt-5">
                <div class="flex items-center">
                    <div class="font-semibold text-lg text-black dark:text-black-dark">{&post.title}</div>
                    <div class="ml-4 flex items-center">
                        {for tags}
                    </div>
                </div>
                <div class="text-sm text-black-400 dark:text-gray-900 mt-1.5 mb-2">
                    {&post.spoiler}
                </div>
                <div class="text-xs text-gray-500 dark:text-gray-450">
                    {&post.created_at}
                </div>
            </div>
        </a>
    }
}
