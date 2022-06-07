use crate::markdown_service::markdown_service::{MarkdownService, PostMetadata};
use crate::posts::POSTS;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use urlencoding::encode;

#[derive(Clone, Debug, PartialEq)]
pub struct Post {
    pub metadata: PostMetadata,
    pub raw_content: &'static str,
    pub desc: String,
    pub modified_time: String,
}

pub struct PostService {
    posts: Vec<Post>,
}

#[derive(Clone)]
pub enum FilterTag {
    All,
    Tag(String),
}

const MAX_DESC_LENGTH: usize = 600;

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
    pub fn new() -> PostService {
        let posts = PostService::read_posts_into_memo();

        PostService { posts }
    }

    pub fn get_posts(&self) -> Vec<Post> {
        self.posts.clone()
    }

    pub fn trim_useless_symbol(content: &'static str) -> String {
        Regex::new(r#"([\n]|```[^`]+```|`[^`]+`)"#)
            .unwrap()
            .replace_all(content, "")
            .into_owned()
    }

    pub fn find_post_by_title(&self, title: &str) -> Option<Post> {
        self.find_post_by_encoded_title(encode(title).to_string().as_str())
    }

    pub fn find_post_by_encoded_title(&self, title: &str) -> Option<Post> {
        self.posts
            .clone()
            .into_iter()
            .find(|post| encode(post.metadata.title.as_str()) == title)
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

    fn read_posts_into_memo() -> Vec<Post> {
        let mut posts = POSTS
            .clone()
            .into_iter()
            .map(|post| {
                let markdown_service = MarkdownService::new(post.content.to_string());
                let metadata = markdown_service.extract_metadata().expect(
                    "Please make sure the post has metadata which declare using block syntax.",
                );
                let parsed_content = PostService::trim_useless_symbol(post.content);
                let parsed_content_length = parsed_content.len();
                let slice_desc_length = if parsed_content_length > MAX_DESC_LENGTH {
                    MAX_DESC_LENGTH
                } else {
                    parsed_content_length
                };
                let desc = parsed_content[..find_char_boundary(&parsed_content, slice_desc_length)]
                    .to_string();
                let modified_secs = (post.modified_time / 1000) as i64;
                let modified_time = NaiveDateTime::from_timestamp(modified_secs, 0);
                let modified_time = modified_time.format("%a, %b %e %Y").to_string();

                Post {
                    metadata,
                    raw_content: post.content,
                    desc,
                    modified_time,
                }
            })
            .collect::<Vec<Post>>();

        posts.sort_by(|a, b| {
            if a.metadata.pined {
                Ordering::Less
            } else {
                a.modified_time.cmp(&b.modified_time)
            }
        });

        posts
    }
}

pub static POST_SERVICE: Lazy<PostService> = Lazy::new(|| PostService::new());
