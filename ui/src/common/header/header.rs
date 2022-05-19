use crate::container::Container;
use crate::header::drawer::Drawer;
use crate::header::drawer_item::DrawerItem;
use crate::image::{Icon, ThemeImage};
use crate::link::Link;
use utils::theme::only_render_on_mobile;
use utils::use_style;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let style = use_style!(
        r"
        width: 100%;

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

        .header {
            height: 70px;
            width: 100%;
            background: var(--base-color);
            position: relative;
            z-index: 6;
        }

        @media (max-width: 600px) {
            .tabs {
                display: none;
            }
        }
    "
    );
    let is_open_drawer_handle = use_state_eq(|| false);
    let handle_drawer_click = {
        let is_open_drawer_handle = is_open_drawer_handle.clone();

        Callback::from(move |_| is_open_drawer_handle.set(!*is_open_drawer_handle))
    };

    html! {
        <div class={style}>
            <Drawer is_open={is_open_drawer_handle}>
                <DrawerItem lnk="/">{"Posts"}</DrawerItem>
                <DrawerItem lnk="/projects">{"Projects"}</DrawerItem>
                <DrawerItem lnk="/about">{"About"}</DrawerItem>
            </Drawer>
            <div class="header">
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
                            {only_render_on_mobile(html! {
                                <Icon source="drawer.svg" size=30 onclick={handle_drawer_click} />
                            })}
                        </div>
                    </div>
                </Container>
            </div>
        </div>
    }
}
