use crate::markdown_service::markdown_service::{MarkdownService, PostMetadata};

use super::posts_container::{Post, POST_CONTAINER};

#[derive(Clone)]
pub struct PostService {
    posts: Vec<Post>,
}

#[derive(Clone)]
pub enum FilterTag {
    All,
    Tag(String),
}

pub fn find_char_boundary(s: &str, index: usize) -> usize {
    if s.len() <= index {
        return index;
    }

    let mut new_index = index;
    while !s.is_char_boundary(new_index) {
        new_index += 1;
    }

    new_index
}

impl PostService {
    pub fn from(key: String) -> PostService {
        let posts = POST_CONTAINER.get_posts_by_key(key);

        PostService { posts }
    }

    pub fn from_all() -> PostService {
        let posts = POST_CONTAINER.get_posts();

        PostService { posts }
    }

    pub fn get_posts(&self) -> Vec<Post> {
        self.posts.clone()
    }

    pub fn find_post_by_filename(&self, filename: &str) -> Option<Post> {
        self.posts
            .clone()
            .into_iter()
            .find(|post| post.filename == filename)
    }

    pub fn get_tags(&self) -> Vec<String> {
        let mut tags = vec![];

        self.posts.iter().for_each(|post| {
            let is_exist = tags.contains(&post.metadata.tag);

            if !is_exist {
                tags.push(post.metadata.tag.clone());
            }
        });

        tags
    }

    pub fn filter_post_by_tag(&self, tag: FilterTag) -> Vec<Post> {
        let posts = self.posts.clone();

        match tag {
            FilterTag::All => posts,
            FilterTag::Tag(tag) => posts
                .into_iter()
                .filter(|post| post.metadata.tag == tag)
                .collect::<Vec<Post>>(),
        }
    }
}
