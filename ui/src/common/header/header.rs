use crate::contact::ContactType;
use crate::container::Container;
use crate::header::drawer::Drawer;
use crate::header::drawer_item::DrawerItem;
use crate::image::{Icon, ThemeImage};
use crate::link::Link;
use crate::modal::{modal::Modal, modal_content::ModalContent};
use crate::theme_selector::ThemeSelector;
use router::RootRoutes;
use utils::resource::with_assets;
use utils::theme::only_render_on_mobile;
use utils::use_style;
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let style = use_style!(
        r"
        width: 100%;

        .wrapper {
            height: 56px;
            justify-content: space-between;
        }

        .wrapper, .tabs, .left, .right, .setting-icon {
            display: flex;
            align-items: center;
        }

        .left img {
            height: 16px;
        }

        .tabs {
            margin-left: 88px;
        }

        .tabs a {
            font-size: 14px;
            margin: 0 15px;
        }

        .setting-icon {
            margin-right: 19px;
        }

        .header {
            height:56px;
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
    let control_theme_style = use_style!(
        r"
        display: flex;
        flex-direction: column;
        align-items: center;

        .control-theme__text {
            width: 100%;
            text-align: left;
            font-size: 14px;
            color: var(--sub-text-color);
        }

        .control-theme__img {
            width: 126px;
        }

        @media (max-width: 600px) {
            .control-theme__text {
                font-size: 12px;
            }
        }
    "
    );
    let is_open_drawer_handle = use_state_eq(|| false);
    let is_open_theme_modal = use_state_eq(|| false);
    let handle_drawer_click = {
        let is_open_drawer_handle = is_open_drawer_handle.clone();

        Callback::from(move |_| is_open_drawer_handle.set(!*is_open_drawer_handle))
    };
    let handle_setting_click = {
        let is_open_theme_modal = is_open_theme_modal.clone();

        Callback::from(move |_| is_open_theme_modal.set(!*is_open_theme_modal))
    };

    html! {
        <div class={style}>
            <Drawer is_open={is_open_drawer_handle}>
                <DrawerItem lnk={RootRoutes::Home}>{"Posts"}</DrawerItem>
                <DrawerItem lnk={RootRoutes::Projects}>{"Projects"}</DrawerItem>
                <DrawerItem lnk={RootRoutes::About}>{"About"}</DrawerItem>
                <DrawerItem lnk={RootRoutes::Links}>{"Links"}</DrawerItem>
            </Drawer>
            <div class="header">
                <Container>
                    <div class="wrapper">
                        <div class="left">
                            <Link href={RootRoutes::Home}>
                                <ThemeImage source="zzhack_logo.svg" />
                            </Link>
                            <div class="tabs">
                                <Link href={RootRoutes::Home}>{"Posts"}</Link>
                                <Link href={RootRoutes::Projects}>{"Projects"}</Link>
                                <Link href={RootRoutes::About}>{"About"}</Link>
                                <Link href={RootRoutes::Links}>{"Links"}</Link>
                            </div>
                        </div>
                        <div class="right">
                            <Icon source="setting.svg" size=26 onclick={handle_setting_click} />
                            <Link out_href={ContactType::GitHub.into_lnk()}>
                                <Icon source="github.svg" size=26 />
                            </Link>
                            {only_render_on_mobile(html! {
                                <Icon source="drawer.svg" size=26 onclick={handle_drawer_click} />
                            })}
                        </div>
                    </div>
                </Container>
            </div>
            <Modal is_visible={is_open_theme_modal} title="选择一个你喜欢的主题" subtitle="你可以选择 light 和 dark 两种主题色，选择一种你喜欢的主题色吧~">
                <ModalContent>
                    <ThemeSelector />
                    <div class={control_theme_style}>
                        <p class="control-theme__text">{"你可以在任何地方通过开关随时修改你的主题"}</p>
                        <img class="control-theme__img" src={with_assets("switch_theme_guide.png")} />
                    </div>
                </ModalContent>
            </Modal>
        </div>
    }
}
