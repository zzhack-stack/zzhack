use crate::markdown_service::elements::render_code;
use crate::markdown_service::elements::render_github_render_block;
use crate::markdown_service::elements::render_heading;
use crate::markdown_service::elements::render_spotlight;
use crate::markdown_service::elements::GitHubRenderBlock;
use crate::markdown_service::elements::{render_code_block, render_image};
use pulldown_cmark::CodeBlockKind;
use pulldown_cmark::CodeBlockKind::{Fenced, Indented};
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use serde::Deserialize;
use serde_json;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;
use web_sys::window;
use web_sys::Element;

#[derive(Clone)]
pub struct MarkdownService {
    value: String,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct PostMetadata {
    pub cover: String,
    pub tag: String,
    pub title: String,
}

#[derive(Clone)]
enum TraverseKind {
    Spotlight,
    CodeBlock(SyntaxReference),
    GitHubRenderBlock,
    Metadata,
    Image(String),
    Heading(u32),
    Nope,
}

impl MarkdownService {
    pub fn new(input: String) -> MarkdownService {
        return MarkdownService { value: input };
    }

    fn parse_code_block_language_kind(kind: &CodeBlockKind) -> String {
        match kind {
            Indented => "rust".to_string(),
            Fenced(language) => language.to_string(),
        }
    }

    fn parse_source_path(path: &str) -> String {
        let path = Path::new(path);
        let filename = path.file_name().unwrap().to_str().unwrap();

        format!("sources/{}", filename)
    }

    /**
     * The post metadata is looks like:
     *
     * ```metadata
     * {
     * "cover": "https://zzhack.fun",
     * "tag": ["zzhack", "zzhack.fun"]
     * }
     * ```
     */
    pub fn extract_metadata(&self) -> Option<PostMetadata> {
        let mut metadata: Vec<String> = vec![];
        let mut traverse_kind = TraverseKind::Nope;
        let parser = Parser::new_ext(&self.value, Options::empty());

        parser.for_each(|event| match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                let language = MarkdownService::parse_code_block_language_kind(&kind);

                if language == "metadata" {
                    traverse_kind = TraverseKind::Metadata;
                }
            }
            Event::End(Tag::CodeBlock(_)) => {
                if let TraverseKind::Metadata = traverse_kind {
                    traverse_kind = TraverseKind::Nope;
                }
            }
            Event::Text(text) => {
                if let TraverseKind::Metadata = traverse_kind {
                    metadata.push(text.to_string())
                }
            }
            _ => {}
        });

        if metadata.len() == 0 {
            return None;
        }

        let parsed_metadata = metadata.join("");
        let mut metadata: PostMetadata = serde_json::from_str(parsed_metadata.as_str()).unwrap();

        metadata.cover = MarkdownService::parse_source_path(&metadata.cover);

        return Some(metadata);
    }

    pub fn parse(&self, theme: &'static str) -> String {
        let mut traverse_kind = TraverseKind::Nope;

        let mut codes: Vec<String> = Vec::new();
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes[theme];
        let parser = Parser::new_ext(&self.value, Options::empty())
            .map(|event| match event {
                Event::Start(Tag::Image(..)) => {
                    traverse_kind = TraverseKind::Image("".into());
                    event
                }
                Event::Start(Tag::Heading(level)) => {
                    traverse_kind = TraverseKind::Heading(level);
                    event
                }
                Event::End(Tag::Heading(..)) => {
                    traverse_kind = TraverseKind::Nope;
                    event
                }
                Event::End(Tag::Image(kind, url, title)) => {
                    if let TraverseKind::Image(alt) = traverse_kind.clone() {
                        traverse_kind = TraverseKind::Nope;

                        return Event::Html(render_image(url.to_string(), alt).into());
                    }

                    traverse_kind = TraverseKind::Nope;

                    Event::End(Tag::Image(kind, url, title))
                }
                Event::Code(code) => Event::Html(render_code(code.to_string()).into()),
                Event::Start(Tag::CodeBlock(kind)) => {
                    let language = MarkdownService::parse_code_block_language_kind(&kind);

                    if language == "github" {
                        traverse_kind = TraverseKind::GitHubRenderBlock;
                        return Event::Start(Tag::CodeBlock(kind));
                    };

                    if language == "spotlight" {
                        traverse_kind = TraverseKind::Spotlight;
                        return Event::Start(Tag::CodeBlock(kind));
                    }

                    let syntax = match ss.find_syntax_by_name(language.as_ref()) {
                        Some(syntax) => syntax,
                        None => ss.find_syntax_by_extension("rs").unwrap(),
                    };
                    traverse_kind = TraverseKind::CodeBlock(syntax.clone());

                    Event::Start(Tag::CodeBlock(kind))
                }
                Event::End(Tag::CodeBlock(code)) => {
                    let parsed_code = codes.join("");
                    let event = match traverse_kind.clone() {
                        TraverseKind::CodeBlock(syntax) => {
                            let html = highlighted_html_for_string(
                                parsed_code.as_str(),
                                &ss,
                                &syntax,
                                theme,
                            );
                            Event::Html(render_code_block(html).into())
                        }
                        TraverseKind::GitHubRenderBlock => {
                            let github_render_block: GitHubRenderBlock =
                                serde_json::from_str(parsed_code.as_str()).unwrap();
                            Event::Html(render_github_render_block(github_render_block).into())
                        }
                        TraverseKind::Spotlight => {
                            Event::Html(render_spotlight(parsed_code.as_str()).into())
                        }
                        _ => Event::End(Tag::CodeBlock(code)),
                    };

                    // reset codes
                    codes = Vec::new();
                    traverse_kind = TraverseKind::Nope;

                    event
                }
                Event::Text(text) => {
                    let empty_str_event = Event::Text("".into());
                    let parsed_text = text.to_string();

                    match traverse_kind {
                        TraverseKind::Image(_) => {
                            traverse_kind = TraverseKind::Image(parsed_text);

                            empty_str_event
                        }
                        TraverseKind::CodeBlock(_)
                        | TraverseKind::GitHubRenderBlock
                        | TraverseKind::Spotlight => {
                            codes.push(parsed_text);

                            empty_str_event
                        }
                        TraverseKind::Heading(level) => {
                            Event::Html(render_heading(parsed_text, level).into())
                        }
                        _ => Event::Text(text),
                    }
                }
                _ => event,
            })
            .filter(|event| match event {
                Event::Start(Tag::CodeBlock(_)) | Event::End(Tag::CodeBlock(_)) => false,
                Event::Start(Tag::Image(..)) | Event::End(Tag::Image(..)) => false,
                Event::Start(Tag::Heading(..)) | Event::End(Tag::Heading(..)) => false,
                _ => true,
            });

        let mut html_output: String = String::with_capacity(self.value.len() * 3 / 2);

        html::push_html(&mut html_output, parser);

        html_output
    }

    pub fn parse_to_element(&self, theme: &'static str) -> Element {
        let window = window().unwrap();
        let div = window.document().unwrap().create_element("div").unwrap();
        let parsed_html = self.parse(theme);

        div.set_inner_html(parsed_html.as_str());

        div
    }
}
