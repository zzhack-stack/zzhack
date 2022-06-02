#[derive(PartialEq, Clone, Debug)]
pub enum PostCardSize {
    Small,
    Large,
}

impl From<String> for PostCardSize {
    fn from(size: String) -> Self {
        match size.as_str() {
            "small" => PostCardSize::Small,
            "large" => PostCardSize::Large,
            _ => PostCardSize::Small,
        }
    }
}
