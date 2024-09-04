use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TagProps {
    pub children: Html,
    pub color: AttrValue,
}

#[function_component]
pub fn Tag(props: &TagProps) -> Html {
    let color = props.color.clone();
    let dynamic_color = format!("bg-[{}]", "#ff0000");
    let a = format!(
        "text-xs mr-1.5 px-1.5 bg-opacity-25 rounded flex justify-center items-center {}",
        dynamic_color
    );

    html! {
        <div class={a}>{props.children.clone()}</div>
    }
}
