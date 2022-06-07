use std::*;

#[derive(Clone)]
pub struct PostFile {
    pub content: &'static str,
    pub modified_time: u128
}

pub static POSTS: [PostFile; 1] = [
    PostFile {
    content: include_str!("../../posts/build_blog.md"),
    modified_time: 1654607237289
},

];
