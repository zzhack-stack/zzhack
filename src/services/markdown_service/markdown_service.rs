use crate::console_log;
use crate::services::markdown_service::elements::render_code;
use crate::services::markdown_service::elements::render_heading;
use crate::services::markdown_service::elements::render_text;
use crate::services::markdown_service::elements::{render_code_block, render_image};
use crate::utils::theme::by_theme;
use pulldown_cmark::CodeBlockKind::{Fenced, Indented};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;
use web_sys::Element;

pub struct MarkdownService {
    value: String,
}

#[derive(Clone)]
enum TraverseKind {
    CodeBlock(SyntaxReference),
    Image(String),
    Heading(u32),
    Nope,
}

impl MarkdownService {
    pub fn new(input: String) -> MarkdownService {
        return MarkdownService { value: input };
    }

    pub fn parse(&self, input: &str, theme: &'static str) -> String {
        // let mut is_code_block = false;
        // let mut is_img = false;
        let mut traverse_kind = TraverseKind::Nope;

        let mut codes: Vec<String> = Vec::new();
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes[theme];

        let parser = Parser::new_ext(input, Options::empty())
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
                    let language = match kind.clone() {
                        Indented => String::from("rust"),
                        Fenced(language) => language.to_string(),
                    };
                    let syntax = match ss.find_syntax_by_name(language.as_str()) {
                        Some(syntax) => syntax,
                        None => ss.find_syntax_by_extension("rs").unwrap(),
                    };
                    traverse_kind = TraverseKind::CodeBlock(syntax.clone());

                    Event::Start(Tag::CodeBlock(kind))
                }
                Event::End(Tag::CodeBlock(code)) => {
                    if let TraverseKind::CodeBlock(syntax) = traverse_kind.clone() {
                        let html = highlighted_html_for_string(
                            codes.join("").as_str(),
                            &ss,
                            &syntax,
                            theme,
                        );
                        traverse_kind = TraverseKind::Nope;
                        // reset codes
                        codes = Vec::new();
                        return Event::Html(render_code_block(html).into());
                    }

                    traverse_kind = TraverseKind::Nope;

                    Event::End(Tag::CodeBlock(code))
                }
                Event::Text(text) => {
                    let empty_str_event = Event::Text("".into());
                    let parsed_text = text.to_string();

                    match traverse_kind {
                        TraverseKind::Image(_) => {
                            traverse_kind = TraverseKind::Image(parsed_text);

                            empty_str_event
                        }
                        TraverseKind::CodeBlock(_) => {
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

        let mut html_output: String = String::with_capacity(input.len() * 3 / 2);

        html::push_html(&mut html_output, parser);

        html_output
    }

    pub fn parse_to_element(&self, theme: &'static str) -> Element {
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_inner_html(self.parse(self.value.as_str(), theme).as_str());

        div
    }
}
