use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LoadMoreProps {
    pub children: Html,
    #[prop_or_default]
    pub class: String,
}

#[function_component]
pub fn OutlineCard(props: &LoadMoreProps) -> Html {
    html! {
        <div class={format!("rounded-6xl bg-white-600 p-3 dark:bg-gray-600 {}", props.class)}>
            <div class="rounded-4xl bg-white dark:bg-white-dark">
               {props.children.clone()}
            </div>
        </div>
    }
}
