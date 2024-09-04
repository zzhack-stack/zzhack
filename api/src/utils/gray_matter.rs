use gray_matter::{engine::YAML, Matter, ParsedEntity};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Tag {
    Text(String),
    CustomColor { text: String, color: String },
}

impl Tag {
    pub fn text(&self) -> String {
        match self {
            Tag::Text(text) => text,
            Tag::CustomColor { text, .. } => text,
        }
        .to_string()
    }
}

impl Into<String> for Tag {
    fn into(self) -> String {
        self.text()
    }
}

#[derive(Deserialize, Debug)]
pub struct PostFrontMatter {
    pub title: String,
    pub spoiler: String,
    pub tags: Option<Vec<Tag>>,
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
