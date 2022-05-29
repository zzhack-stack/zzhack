use crate::post_card_header::PostCardHeader;
use services::post_service::Post;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostCardProps {
    pub post: Post,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let style = use_style!(
        r"
        width: 312px;
        height: 413px;
        background: var(--base-color);
        box-shadow: 0px 7px 43px 0px var(--card-shadow-color);
        border-radius: 22px;
        box-sizing: border-box;
        padding: 22px;
        margin-bottom: 79px;

        .cover {
            width: 295px;
            height: 168px;
            border-radius: 10px;
            margin: 17px -14px;
        }

        .post-preview__title {
            font-size: 19px;
            font-weight: bold;
            color: var(--text-color);
            line-height: 26px;
        }

        .post-preview__body {
            width: 100%;
            height: 66px;
            word-break: break-all;
            text-overflow: ellipsis;
            line-height: 22px;
            display: -webkit-box;
            -webkit-line-clamp: 3;
            -webkit-box-orient: vertical;
            overflow: hidden;
            margin-top: 11px;
            color: var(--sub-text-color);
        }
    "
    );

    html! {
        <div class={style}>
            <PostCardHeader />
            <img class="cover" src={props.post.metadata.cover.clone()} />
            <div class="post-preview">
                <div class="post-preview__title">
                    {&props.post.metadata.title}
                </div>
                <div class="post-preview__body">
                    {&props.post.desc}
                </div>
            </div>
        </div>
    }
}
