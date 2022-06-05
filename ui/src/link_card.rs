use crate::link::Link;
use services::links_service::links_service::LinkData;
use stylist::style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LinkCardProps {
    pub link: LinkData,
}

#[function_component(LinkCard)]
pub fn link_card(props: &LinkCardProps) -> Html {
    let style = style!(
        r"
        width: 250px;
        height: 98px;
        box-shadow: 0px 8px 50px 0px var(--card-shadow-color);
        background: var(--base-color);
        border-radius: 20px;
        margin: 0 15px;
        display: flex;
        align-items: center;
        padding: 10px 25px;
        box-sizing: border-box;
        margin-bottom: 25px;
        

        .logo {
            width: 55px;
            height: 55px;
            border-radius: 10px;
        }

        .link-info {
            margin-left: 12px;
            height: 60px;
        }

        .link-desc {
            font-size: 13px;
            color: var(--sub-text-color);
            line-height: 18px;
            -webkit-line-clamp: 2;
            -webkit-box-orient: vertical;
            overflow: hidden;
            height: 36px;
            text-overflow: ellipsis;
            display: -webkit-box;
            width: 150px;
        }

        .link-name {
            width: 150px;
            text-overflow: ellipsis;
            overflow: hidden;
            white-space: pre;
        }

        @media (max-width: 600px) {
            width: 100%;
            margin: auto;
            margin-bottom: 20px;

            .link-desc {
                width: 100%;
            }

            .link-name {
                width: 100%;
            }
        }
    "
    )
    .unwrap();

    html! {
        <Link out_href={props.link.addr.clone()}>
            <div class={style}>
                <img class="logo" src={props.link.logo.clone()} />
                <div class="link-info">
                    <div class="link-name">{&props.link.name}</div>
                    <div class="link-desc">{&props.link.desc}</div>
                </div>
            </div>
        </Link>
    }
}
