use gray_matter::{engine::YAML, Matter};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostFrontMatter {
    pub title: String,
    pub spoiler: String,
}

pub fn parse_post_gray_matter(content: &str) -> PostFrontMatter {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(content);

    result
        .data
        .unwrap()
        .deserialize::<PostFrontMatter>()
        .unwrap()
}
