use std::*;

pub struct PostFile {
    pub content: &'static str,
    pub modified_time: u128
}

pub static POSTS: &[PostFile; 2] = &[
    PostFile {
    content: include_str!("../../posts/bar.md"),
    modified_time: 1654101374572
},
PostFile {
    content: include_str!("../../posts/foo.md"),
    modified_time: 1654101496839
},

];
