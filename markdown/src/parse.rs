use pulldown_cmark::{html, Parser};

// Parse markdown to html string
// features:
// - Render JSX component
pub fn parse_markdown(markdown_content: &str) -> String {
    let parser = Parser::new(markdown_content).map(|event| match event {
        _ => event,
    });

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}
