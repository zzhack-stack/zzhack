use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub children: Html,
}

#[function_component]
pub fn Tag(props: &TagProps) -> Html {
    html! {
        <span class="pixel-badge mr-2 px-2 py-0.5 text-xs text-black-500 text-white bg-black">{props.children.clone()}</span>
    }
}
