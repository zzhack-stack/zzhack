use pulldown_cmark::{html, Parser};
use web_sys::Element;

pub struct MarkdownService {
    value: String,
}

impl MarkdownService {
    pub fn new(input: String) -> MarkdownService {
        return MarkdownService { value: input };
    }

    pub fn parse(&self, input: &str) -> String {
        let parser = Parser::new(input);
        let mut html_output: String = String::with_capacity(input.len() * 3 / 2);

        html::push_html(&mut html_output, parser);

        html_output
    }

    pub fn parse_to_element(&self) -> Element {
        let div = yew::utils::document().create_element("div").unwrap();
        div.set_inner_html(self.parse(self.value.as_str()).as_str());

        div
    }
}
