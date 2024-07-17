use shared::links::LinksConfig;
use yew::prelude::*;

#[cfg(feature = "ssr")]
async fn fetch_links() -> LinksConfig {
    use crate::http::HTTP;

    HTTP::new()
        .get("/links")
        .await
        .unwrap()
        .json::<LinksConfig>()
        .await
        .unwrap()
}

#[function_component]
pub fn Content() -> HtmlResult {
    let prepared_links =
        use_prepared_state!((), async move |_| -> LinksConfig { fetch_links().await })?.unwrap();
    let rendered_links = prepared_links.links.iter().map(|link| {
        html! {
            <div>
                <img src={(&link).avatar.clone()} />
                <div>{&link.name}</div>
                <div>{&link.description}</div>
            </div>
        }
    });

    Ok(html! {
        {for rendered_links}
    })
}

#[function_component]
pub fn Links() -> Html {
    let fallback = html! {<div>{"Loading..."}</div>};

    html! {
        <Suspense fallback={fallback}>
           <Content />
        </Suspense>
    }
}
