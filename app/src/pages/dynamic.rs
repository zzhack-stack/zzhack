use yew::prelude::*;

use crate::utils::inner_html::parse_str_to_element;

#[cfg(feature = "ssr")]
async fn fetch_dynamic_post_rendered_content(path: &str) -> String {
    use crate::http::HTTP;

    HTTP::new()
        .get(&format!("/dynamic/post/{path}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

#[derive(Properties, PartialEq)]
pub struct DynamicProps {
    pub path: String,
}

#[function_component]
pub fn Content(props: &DynamicProps) -> HtmlResult {
    let path = props.path.clone();
    let prepared_dynamic_post_content = use_prepared_state!((), async move |_| -> String {
        fetch_dynamic_post_rendered_content(&path).await
    })?
    .unwrap();

    Ok(html! {
        <div>
        {parse_str_to_element(&prepared_dynamic_post_content)}

            </div>
    })
}

#[function_component]
pub fn Dynamic(props: &DynamicProps) -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
       <Suspense fallback={fallback}>
           <Content path={props.path.clone()} />
       </Suspense>
    }
}
