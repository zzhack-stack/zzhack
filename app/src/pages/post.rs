use shared::post::PostDetail;
use yew::prelude::*;

use crate::utils::inner_html::parse_str_to_element;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: usize,
}

#[cfg(feature = "ssr")]
async fn fetch_post_detail(id: usize) -> PostDetail {
    use crate::http::HTTP;

    HTTP::new()
        .get(&format!("/posts/{id}"))
        .await
        .unwrap()
        .json::<PostDetail>()
        .await
        .unwrap()
}

#[function_component]
fn Content(props: &PostProps) -> HtmlResult {
    let id = props.id.clone();
    let parepared_post_detail = use_prepared_state!((), async move |_| -> PostDetail {
        fetch_post_detail(id).await
    })?
    .unwrap();
    let element = parse_str_to_element(&parepared_post_detail.content);

    Ok(html! {
        element
    })
}

#[function_component]
pub fn Post(props: &PostProps) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense fallback={fallback}>
           <Content id={props.id} />
        </Suspense>
    }
}
