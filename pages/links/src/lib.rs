use services::links_service::links_service::LINKS_SERVICE;
use stylist::style;
use ui::link_card::LinkCard;
use yew::prelude::*;

#[function_component(Links)]
pub fn links() -> Html {
    let style = style!(
        r"
        .banner {
            position: relative;
            margin-top: 0px;
        }

        .banner > img {
            width: 100%;
            height: 165.59px;
        }

        .banner__links {
            position: absolute;
            top: 45px;
            left: 35px;
        }

        .banner__links-title {
            color: #86a8e7;
            font-size: 18px;
        }

        .banner__links-desc {
            color: rgba(255, 255, 255, 0.81);
            font-size: 14px;
            margin-top: 10px;
            width: 500px;
            line-height: 12px;
        }

        .banner__links-desc > a {
            color: var(--blue);
            font-size: 14px;
        }

        .links {
            margin: 0 -15px;
            display: flex;
            flex-wrap: wrap;
            margin-top: 50px;
        }

        @media (max-width: 600px) {
            .banner__links {
                top: auto;
                left: auto;
                bottom: 30px;
                width: 100%;
                word-break: break-all;
                padding: 0 20px;
                box-sizing: border-box;
            }

            .banner__links-desc {
                width: 100%;
                line-height: 20px;
            }

            .banner > img {
                height: 323.69px;
            }

            .links {
                width: 100%;
                margin: auto;
                margin-top: 20px;
                margin-bottom: 100px;
            }

            .links a {
                width: 100%;
            }
        }
    "
    )
    .unwrap();

    html! {
        <div class={style}>
            <div class="banner">
                <div class="banner__links">
                    <div class="banner__links-title">{"🔗 Links"}</div>
                </div>
            </div>
            <br/>
            <div class="links">
                {
                    LINKS_SERVICE.get_links_data().iter().map(|data| {
                        html! {
                            <LinkCard link={data.clone()} />
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
