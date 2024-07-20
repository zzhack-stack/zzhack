use gray_matter::{engine::YAML, Matter, ParsedEntity};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostFrontMatter {
    pub title: String,
    pub spoiler: String,
    pub tags: Option<Vec<String>>,
}

fn parse_gray_matter(content: &str) -> ParsedEntity {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(content);

    result
}

pub fn get_post_front_matter(content: &str) -> PostFrontMatter {
    parse_gray_matter(content)
        .data
        .expect("Failed to parse front matter")
        .deserialize::<PostFrontMatter>()
        .unwrap()
}

// Return post content without front matter
pub fn get_post_content(content: &str) -> String {
    parse_gray_matter(content).content
}
