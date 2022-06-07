use crate::link::Link;
use crate::post_card_header::PostCardHeader;
use router::RootRoutes;
use services::post_service::post_service::Post;
use stylist::style;
use urlencoding::encode;
use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct PostCardLargeProps {
    pub post: Post,
}

#[function_component(PostCardLarge)]
pub fn post_card_large(props: &PostCardLargeProps) -> Html {
    let style = style!(
        r#"
        height: 330px;
        display: flex;
        border-radius: 17px;
        background: var(--base-color);
        overflow: hidden;
        box-shadow: 0px 7px 43px 0px var(--card-shadow-color);
        margin: 0 18px;
        margin-bottom: 35px;

        .cover {
            flex-shrink: 0;
            width: 446.51px;
            height: 330px;
            background-image: url("${cover}");
            background-repeat: no-repeat;
            background-size: cover;
            background-position: 50% 50%;
        }

        .post-preview {
            margin-left: 40px;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            padding: 23px 25px 10px 0;
        }

        .post-preview__body {
            width: 100%;
            word-break: break-all;
            font-size: 14.48px;
            color: var(--sub-text-color);
            line-height: 25px;
            -webkit-line-clamp: 6;
            -webkit-box-orient: vertical;
            overflow: hidden;
            height: 156px;
            text-overflow: ellipsis;
            display: -webkit-box;
        }

        .post-preview__title {
            font-size: 23px;
            font-weight: 600;
            color: var(--text-color);
            line-height: 33px;
            margin-top: 25px;
            margin-bottom: 10px;
        }

        .modified-at {
            display: flex;
            width: 100%;
            justify-content: flex-end;
            color: var(--tip-color);
            font-size: 10px;
            line-height: 15px;
            margin-top: 10px;
        }
    "#,
        cover = props.post.metadata.cover.clone()
    )
    .unwrap();
    let post_encoded_title = encode(props.post.metadata.title.as_str());

    html! {
        <Link href={RootRoutes::Post{title: post_encoded_title.to_string()}}>
        <div class={style}>
            <div class="cover" />
            <div class="post-preview">
                <div>
                    <PostCardHeader label={props.post.metadata.tag.clone()} />
                    <div class="post-preview__title">{&props.post.metadata.title}</div>
                    <div class="post-preview__body">{&props.post.desc}</div>
                </div>
                <div class="modified-at">{&props.post.modified_time}</div>
            </div>
        </div>
        </Link>
    }
}
