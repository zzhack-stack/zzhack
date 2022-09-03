use crate::label::Label;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostCardHeaderProps {
    pub label: String,
}

#[function_component(PostCardHeader)]
pub fn post_card_header(props: &PostCardHeaderProps) -> Html {
    let style = use_style!(
        r"
        display: flex;
        align-items: center;
        width: 100%;
        justify-content: space-between;


        .avatar {
            width: 32px;
            height: 32px;
        }

        .author {
            display: flex;
            align-items: center;
            margin-right: 8px;
        }

        .author-name {
            font-weight: bold;
            font-size: 17px;
            color: var(--text-color);
            margin-left: 8px;
        }
    "
    );

    html! {
        <div class={style}>
            <div class="author">
                <img src="/images/rsz_pfp.png" class="avatar" alt="avatar" />
                <span class="author-name">{"andnasnd"}</span>
            </div>
            <Label text={props.label.clone()} />
        </div>
    }
}
