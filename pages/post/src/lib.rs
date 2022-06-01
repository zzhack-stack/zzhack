use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub encoded_title: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    html! {"Posts"}
}
