use std::{fs::read_to_string, path::PathBuf};

use yew::prelude::*;
use yew_router::components::Redirect;

use crate::{routes::Routes, utils::post_path::get_markdown_path};

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub name: String,
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {
    let markdown_path = get_markdown_path(PathBuf::from("../posts").join(&props.name));

    match read_to_string(markdown_path) {
        Ok(content) => html! {content},
        Err(_) => {
            html! {
                <Redirect<Routes> to={Routes::NotFound} />
            }
        }
    }
}
