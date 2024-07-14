use std::{fs::read_to_string, path::PathBuf};

use yew::prelude::*;
use yew_router::components::Redirect;

use crate::routes::Routes;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub name: String,
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {
    // let markdown_path = get_markdown_path(PathBuf::from("../posts").join(&props.name));
    html! {"Post"}

    // match read_to_string(markdown_path) {
    //     Ok(content) => {
    //         let post_content = get_post_content(&content);
    //
    //         html! {<div class="prose lg:prose-xl">{parse_markdown_as_html(&post_content)}</div>}
    //     }
    //     Err(_) => {
    //         html! {
    //             <Redirect<Routes> to={Routes::NotFound} />
    //         }
    //     }
    // }
}
