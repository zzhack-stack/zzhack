use std::*;

#[derive(Clone)]
pub struct PostFile {
    pub content: &'static str,
    pub modified_time: u128,
    pub filename: &'static str
}

pub static POSTS: [PostFile; 2] = [
    PostFile {
    content: include_str!("../../posts/build_blog.md"),
    modified_time: 1654609915763,
    filename: "build_blog"
},
PostFile {
    content: include_str!("../../posts/add_links.md"),
    modified_time: 1654855511389,
    filename: "add_links"
},

];
