use std::*;

#[derive(Clone)]
pub struct PostFile {
    pub content: &'static str,
    pub modified_time: u128
}

pub static POSTS: [PostFile; 4] = [
    PostFile {
    content: include_str!("../../posts/bar.md"),
    modified_time: 1654101374572
},
PostFile {
    content: include_str!("../../posts/foo.md"),
    modified_time: 1654153316337
},
PostFile {
    content: include_str!("../../posts/asd.md"),
    modified_time: 1654489203129
},
PostFile {
    content: include_str!("../../posts/test.md"),
    modified_time: 1654489296116
},

];
