use crate::container::Container;
use crate::image::{Icon, ThemeImage};
use crate::link::Link;
use utils::use_style;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let style = use_style!(
        r"
        height: 70px;
        width: 100%;
        background: var(--base-color);
        
        .wrapper {
            height: 70px;
            justify-content: space-between;
        }

        .wrapper, .tabs, .left, .right, .setting-icon {
            display: flex;
            align-items: center;
        }

        .tabs {
            margin-left: 88px;
        }

        .tab {
            margin: 0 15px;
        }

        .setting-icon {
            margin-right: 19px;
        }

        @media (max-width: 600px) {
            .tabs {
                display: none;
            }
        }
    "
    );

    html! {
        <div class={style}>
            <Container>
                <div class="wrapper">
                    <div class="left">
                        <ThemeImage source="zzhack_logo.svg" />
                        <div class="tabs">
                            <div class="tab">
                                <Link href="">{"Posts"}</Link>
                            </div>
                            <div class="tab">
                                <Link href="">{"Projects"}</Link>
                            </div>
                            <div class="tab">
                                <Link href="">{"About"}</Link>
                            </div>
                        </div>
                    </div>
                    <div class="right">
                        <Icon source="setting.svg" size=30 />
                        <Icon source="github.svg" size=30 />
                    </div>
                </div>
            </Container>
        </div>
    }
}
