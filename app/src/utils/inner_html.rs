use yew::virtual_dom::VNode;
use yew::Html;

pub fn parse_str_to_element(content: &str) -> Html {
    VNode::from_html_unchecked(content.to_string().into())
}
