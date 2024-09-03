use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub children: Html,
}

#[function_component]
pub fn Tag(props: &TagProps) -> Html {
    html! {
        <div class="text-xs px-2 rounded flex justify-center items-center">{props.children.clone()}</div>
    }
}
