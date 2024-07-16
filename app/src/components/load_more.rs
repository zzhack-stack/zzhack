use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LoadMoreProps {
    pub onload: Callback<()>,
}

#[function_component]
pub fn LoadMore(props: &LoadMoreProps) -> Html {
    let onload = props.onload.clone();
    let handle_load_more_click = Callback::from(move |_| {
        onload.emit(());
    });

    html! {
        <button onclick={handle_load_more_click}>{"Load more"}</button>
    }
}
