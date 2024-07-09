use crate::utils::{gray_matter::parse_post_gray_matter, post_path::get_markdown_path};
use std::fs::{read_dir, read_to_string};
use yew::prelude::*;

struct PostBriefInfo {
    name: String,
    create_at: String,
    spoiler: String,
}

fn get_posts_list() -> Vec<PostBriefInfo> {
    read_dir("../posts")
        .unwrap()
        .map(|post| {
            let post_entry = post.unwrap();
            let content = read_to_string(get_markdown_path(post_entry.path())).unwrap();
            let gray_matter = parse_post_gray_matter(&content);

            PostBriefInfo {
                name: gray_matter.title,
                spoiler: gray_matter.spoiler,
                create_at: "Tue Jul 09 2024 20:48:31 GMT+0800 (China Standard Time)".to_string(),
            }
        })
        .collect::<Vec<PostBriefInfo>>()
}

#[function_component]
pub fn Posts() -> Html {
    let posts_list = get_posts_list();
    let rendered_posts_list = posts_list.iter().map(|post_brief_info| {
        html! {
            <div>
                <h2>{&post_brief_info.name}</h2>
                <p>{&post_brief_info.spoiler}</p>
                <p>{&post_brief_info.create_at}</p>
            </div>
        }
    });

    html! {
        <div>
            {for rendered_posts_list}
        </div>
    }
}
