use crate::console_log;
use crate::services::markdown_service::elements::heading;
use crate::utils::theme::by_theme;
use pulldown_cmark::CodeBlockKind::{Fenced, Indented};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use web_sys::Element;

pub struct MarkdownService {
    value: String,
}

impl MarkdownService {
    pub fn new(input: String) -> MarkdownService {
        return MarkdownService { value: input };
    }

    pub fn parse(&self, input: &str, theme: &'static str) -> String {
        let mut is_code_block = false;
        let mut to_highlight = String::new();
        let ss = SyntaxSet::load_defaults_newlines();
        let mut syntax = ss.find_syntax_by_extension("rs").unwrap();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes[theme];

        let parser = Parser::new_ext(input, Options::empty())
            .map(|event| match event {
                Event::Start(Tag::Heading(..)) => Event::Html(heading::HEADING_START.into()),
                Event::End(Tag::Heading(..)) => Event::Html(heading::HEADING_END.into()),
                Event::Start(Tag::CodeBlock(kind)) => {
                    // console_log!("{:?}", kind);
                    let language = match kind.clone() {
                        Indented => String::from("rust"),
                        Fenced(language) => language.to_string(),
                    };

                    syntax = match ss.find_syntax_by_name(language.as_str()) {
                        Some(syntax) => syntax,
                        None => ss.find_syntax_by_extension("rs").unwrap(),
                    };
                    is_code_block = true;
                    Event::Start(Tag::CodeBlock(kind))
                }
                Event::End(Tag::CodeBlock(_)) => {
                    is_code_block = false;
                    event
                }
                Event::Text(text) => {
                    let a: String = text.to_string();

                    if a.trim().len() == 0 {
                        return Event::Text(text);
                    }

                    if is_code_block {
                        let html = highlighted_html_for_string(
                            text.to_string().as_str(),
                            &ss,
                            syntax,
                            theme,
                        );
                        return Event::Html(
                            format!("<div class='markdown-code'>{}</div>", html).into(),
                        );
                    };
                    Event::Text(text)
                }
                _ => event,
            })
            .filter(|event| match event {
                Event::Start(Tag::CodeBlock(_)) | Event::End(Tag::CodeBlock(_)) => false,
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
