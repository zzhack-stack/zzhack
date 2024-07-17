use yew::Html;

#[cfg(target_arch = "wasm32")]
pub fn parse_str_to_element(html_string: &str) -> Html {
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
pub fn parse_str_to_element(content: &str) -> Html {
    use yew::virtual_dom::VNode;

    VNode::from_html_unchecked(content.to_string().into())
}
