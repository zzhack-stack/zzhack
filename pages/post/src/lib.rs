use services::markdown_service::markdown_service::MarkdownService;
use services::post_service::POST_SERVICE;
use stylist::style;
use ui::post_card_header::PostCardHeader;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostProps {
    pub encoded_title: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let post = POST_SERVICE
        .find_post_by_encoded_title(&props.encoded_title)
        .unwrap();
    let style = style!(
        r#"
        width: 660px;
        height: 100%;
        padding: 63px 0;
        margin: auto;

        .post-header {
            width: 180px;
        }

        .modified-at {
            color: var(--tip-color);
            margin-top: 13px;
            font-size: 13px;
        }

        .title {
            font-size: 25px;
            color: var(--text-color);
        }

        .cover {
            width: 660px;
            height: 258px;
            background-image: url("${cover}");
            background-repeat: no-repeat;
            background-size: cover;
            background-position: 50% 50%;
            border-radius: 5px;
            margin: 30px 0;
            transition: all 0.2s ease-in;
            box-shadow: rgba(0, 0, 0, 0.12) 0px 1px 3px, rgba(0, 0, 0, 0.24) 0px 1px 2px;        
        }

        @media (max-width: 600px) {
            width: 100%;

            .cover {
                width: 100%;
                height: 180px;
            }
        }
    "#,
        cover = post.metadata.cover.clone()
    )
    .unwrap();
    let post_body = MarkdownService::new(post.raw_content.clone().to_string());
    let post_body = post_body.parse_to_element("base16-ocean.dark");

    html! {
        <div class={style}>
            <div class="post-header">
                <PostCardHeader label={post.metadata.tag.clone()} />
            </div>
            <div class="modified-at">{&post.modified_time}</div>
            <div class="cover" />
            <h1 class="title">
                {&post.metadata.title}
            </h1>
            <div class="body">
                {Html::VRef(post_body.clone().into())}
            </div>
        </div>
    }
}
