use std::rc::Rc;

use crate::components::{load_more::LoadMore, post_item::PostItem};
use crate::http::HTTP;
use log::info;
use shared::post::{PaginationPostsRes, Post, PostWithTags, Tag};
use yew::{platform::spawn_local, prelude::*};

#[derive(Properties, PartialEq)]
struct PostItemProps {
    title: String,
    spoiler: String,
}

#[cfg(feature = "ssr")]
async fn fetch_posts(
    page_limit: usize,
    page: usize,
) -> PaginationPostsRes<PostWithTags<Post, Tag>> {
    HTTP::new()
        .get(&format!("/posts?page_limit={page_limit}&page={page}"))
        .await
        .unwrap()
        .json::<PaginationPostsRes<PostWithTags<Post, Tag>>>()
        .await
        .unwrap()
}

const PAGE_LIMIT: usize = 10;
const INIT_PAGE: usize = 0;

#[function_component]
pub fn Content() -> HtmlResult {
    let page = use_state(|| INIT_PAGE);
    let page_cloned = page.clone();

    // The prepared_state macro return a Rc object, the Yew fetch data on server side
    // and send this data to browser, then call hydrate, if we want to redefine a state
    // that inited by prepared_state, we should extract value from Rc functor
    let prepared_pagination_posts = use_prepared_state!((), async move |_| -> PaginationPostsRes<
        PostWithTags<Post, Tag>,
    > {
        fetch_posts(PAGE_LIMIT, *page_cloned).await
    })?
    .unwrap();
    let pagination_posts =
        Rc::try_unwrap(prepared_pagination_posts).unwrap_or_else(|rc| (*rc).clone());
    let has_load_more = use_state(|| pagination_posts.has_next);
    let has_load_more_cloned = has_load_more.clone();
    let posts = use_state(|| pagination_posts.posts_with_tags);
    let posts_clone = posts.clone();
    let rendered_posts = posts_clone.iter().map(|post_with_tags| {
        html! {
            <PostItem post={post_with_tags.post.clone()} />
        }
    });

    let handle_load_more_click = Callback::from(move |_| {
        page.set(*page + 1);
        let page = page.clone();
        let posts = posts.clone();
        let has_load_more = has_load_more_cloned.clone();

        // Spawn local is used for convert JS Promise to Future
        // the block of code which only running in browser
        spawn_local(async move {
            let pagination_posts = fetch_posts(PAGE_LIMIT, *page).await;

            posts.set([(*posts).clone(), pagination_posts.posts_with_tags].concat());
            has_load_more.set(pagination_posts.has_next);
        });
    });

    Ok(html! {
        <div onclick={Callback::from(|_| {info!("World")})}>
            {for rendered_posts}
            if *has_load_more {
                <LoadMore onload={handle_load_more_click} />
            }
        </div>
    })
}

#[function_component]
pub fn Posts() -> HtmlResult {
    let fallback = html! {<div>{"Loading..."}</div>};

    // Any component want to do data fetch, there must have a Suspense component
    // wrap the content component as children, because Yew need a vdom placeholder
    // when start hydrate
    Ok(html! {
        <Suspense fallback={fallback}>
           <Content />
        </Suspense>
    })
}
