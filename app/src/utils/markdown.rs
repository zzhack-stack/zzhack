use pulldown_cmark::{html, Parser};
use yew::{prelude::*, virtual_dom::VNode};

#[cfg(target_arch = "wasm32")]
fn parse_string_to_html(html_string: &str) -> Html {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div.set_inner_html(html_string);

    Html::VRef(div.into())
}

#[cfg(not(target_arch = "wasm32"))]
fn parse_string_to_html(content: &str) -> Html {
    VNode::from_html_unchecked(content.to_string().into())
}

pub fn parse_markdown_as_html(markdown_content: &str) -> Html {
    parse_string_to_html(&parse_markdown(markdown_content))
}

// Parse markdown to html string
// features:
// - Render JSX component
pub fn parse_markdown(markdown_content: &str) -> String {
    let parser = Parser::new(markdown_content).map(|event| {
        println!("{:?}", event);

        match event {
            _ => event,
        }
    });

    let mut html_output = String::new();

    html::push_html(&mut html_output, parser);

    html_output
}
