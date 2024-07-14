// #[cfg(not(target_arch = "wasm32"))]
// use log::info;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct PostItemProps {
    title: String,
    spoiler: String,
}
//
// #[function_component]
// fn PostItem(props: &PostItemProps) -> Html {
//     html! {
//         <div onclick={Callback::from(|_| {
//             info!("Hello World");
//         })}>
//             <p>{props.title.clone()}</p>
//             <p>{props.spoiler.clone()}</p>
//         </div>
//     }
// }

#[function_component]
pub fn Posts() -> Html {
    // let posts_list = get_posts_list();
    // let rendered_posts_list = posts_list.iter().map(|post_brief_info| {
    //     html! {
    //         <div>
    //             <h2>{&post_brief_info.name}</h2>
    //             <p>{&post_brief_info.spoiler}</p>
    //             <p>{&post_brief_info.create_at}</p>
    //             <p>{&post_brief_info.read_minutes}</p>
    //         </div>
    //     }
    // });

    // #[cfg(not(target_arch = "wasm32"))]
    // {
    //     let foo = get_posts_by_page(0, 10).unwrap();
    //     // let posts = get_posts_by_page(0, 10).unwrap().into_iter().map(|post| {
    //     //     let post = post;
    //     //
    //     //     html! {
    //     //         <PostItem title={post.title} spoiler={post.spoiler} />
    //     //     }
    //     // });
    //
    //     return html! {
    //         <>
    //             {VNode::from_html_unchecked(format!("<script>window.posts = {}</script>", serde_json::to_string(&foo.into_iter().collect::<Vec<DatabasePost>>()).unwrap()).into())}
    //         </>
    //     };
    // }
    //
    // #[cfg(target_arch = "wasm32")]
    // {
    //     // let posts = js_sys::Reflect::get(&web_sys::window().unwrap(), &"posts".into()).unwrap();
    //
    //     let posts = get_posts().into_iter().map(|post| {
    //         html! {<PostItem title={post.title.clone()} spoiler={post.spoiler.clone()} />
    //         }
    //     });
    //     // info!("{:?}", posts);
    //
    //     html! {
    //         <div>
    //         <button onclick={Callback::from(|_| {
    //         info!("Hello World!");
    //     })}>{"x"}</button>
    //         {for posts}
    //         </div>
    //     }
    // }

    html! {}
}
