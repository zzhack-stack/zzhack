use crate::console_log;
use crate::routes::app_routes::AppRoutes;
use crate::services::{theme_service::DARK_THEME_KEY, ThemeService};
use crate::utils::theme::by_theme;
use crate::utils::theme::{only_render_on_mobile, only_render_on_pc};
use crate::workers::theme_agent::ThemeAgent;
use crate::AppRouterAnchor;
use css_in_rust::style::Style;
use material_yew::MatIconButton;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct HeaderProps {
    pub tabs: Vec<Tab>,
    pub on_menu_click: Callback<web_sys::MouseEvent>,
}

pub struct Header {
    props: HeaderProps,
    link: ComponentLink<Self>,
    style: Style,
    is_dark_theme: bool,
    theme_agent: Box<dyn Bridge<ThemeAgent>>,
    is_open_drawer: bool,
    theme_service: ThemeService,
}

pub enum HeaderMessage {
    SwitchTheme,
    ChangeTheme,
    ToggleDrawer,
}

#[derive(Clone)]
pub struct Tab {
    pub route: AppRoutes,
    pub name: &'static str,
}

const TABS: [Tab; 3] = [
    Tab {
        name: "Technology",
        route: AppRoutes::Technology,
    },
    Tab {
        name: "Thinking",
        route: AppRoutes::Thinking,
    },
    Tab {
        name: "Fragments",
        route: AppRoutes::Fragments,
    },
];

// fn get_github_oauth_url() -> String {
//     let window = web_sys::window().unwrap();
//     let origin = window.location().href().unwrap();

//     format!(
//         "https://github.com/login/oauth/authorize?client_id=20ac7165581dc3691b9d&redirect_uri=http://localhost:8080/oauth/redirect?origin={}",
//         origin
//     )
// }

// fn find_current_route_index(tabs: Vec<Tab>, current_route: Route) -> u32 {
//     match tabs.iter().position(|tab| {
//         let route: Route = tab.route.clone().into();
//         current_route.contains(route.as_str())
//     }) {
//         Some(i) => i as u32,
//         None => 0,
//     }
// }

impl Header {
    fn with_open_drawer_style(&self, style: &'static str) -> &'static str {
        if self.is_open_drawer {
            style
        } else {
            ""
        }
    }
}

impl Component for Header {
    type Message = HeaderMessage;
    type Properties = HeaderProps;

    fn create(props: HeaderProps, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Header",
            "
            position: relative;
            background: var(--base-color);

            .header-wrapper {
                width: 100%;
                height: 75px;
                display: flex;
                justify-content: space-between;
                align-items: center;
            }

            .zzhack-text {
                font-size: 16px;
                font-weight: 600;
                color: var(--base-text-color);
                line-height: 22px;
                margin-left: 8px;
            }

            .tab-item {
                text-decoration: none;
                transition: all 0.3s;
                margin-right: 24px;
            }

            .tab-item:hover {
                opacity: 0.6;
            }

            .left {
                display: flex;
                justify-content: center;
                align-items: center;
            }

            .right {
                display: flex;
            }

            .logo {
                display: flex;
                align-items: center;
            }

            .logo-icon {
                margin-bottom: 10px;
                width: 32px;
                height: 32px;
            }

            .tabs {
                margin-left: 50px;
            }

            .menu {
                margin-left: 10px;
            }

            .drawer {
                position: absolute;
                width: 100%;
                padding: 16px 0;
                left: 0;
                top: 100%;
                background: var(--base-color);
                box-sizing: border-box;
                transition: all 0.3s;
                transform: translateX(-100%);
                z-index: 2;
                border-top: 1px solid var(--border-color);
            }

            .drawer-tab-item {
                height: 40px;
                line-height: 40px;
                padding: 0 14px;
                box-sizing: border-box;
                transition: all 0.3s;
            }

            .drawer-tab-item:hover {
                background: var(--technology-hover-color);
            }

            .drawer-tab-item__link {
                text-decoration: none;
                transition: all 0.3s;
            }

            .drawer-mask {
                display: none;
                position: fixed;
                top: 0;
                left: 0;
                height: 100%;
                width: 100%;
                z-index: 1;
                margin-top: 75px;
                transition: 0.3s all;
            }

            @media (max-width: 600px) {
                .container {
                    overflow: initial;
                }
            }
        ",
        )
        .unwrap();

        let theme_service = ThemeService::new();
        let theme = theme_service.theme.clone();
        let theme_agent = ThemeAgent::bridge(link.callback(|_| HeaderMessage::ChangeTheme));

        Self {
            props,
            link,
            style,
            is_dark_theme: theme == DARK_THEME_KEY,
            theme_agent,
            is_open_drawer: false,
            theme_service,
        }
    }

    fn update(&mut self, msg: HeaderMessage) -> bool {
        match msg {
            HeaderMessage::ToggleDrawer => {
                self.is_open_drawer = !self.is_open_drawer;
                true
            }
            HeaderMessage::SwitchTheme => {
                console_log!("asdadsdsa");
                self.is_dark_theme = !self.is_dark_theme;
                self.theme_service.switch(self.is_dark_theme);

                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        let open_drawer_styles = "transform: translateX(0px);";
        let open_drawer_mask_styles = "display: block; background: var(--mask-color);";

        html! {
            <div class=self.style.clone().to_string()>
                <div ontouchstart=self.link.callback(|_| HeaderMessage::ToggleDrawer) onclick=self.link.callback(|_| HeaderMessage::ToggleDrawer) class="drawer-mask" style={self.with_open_drawer_style(open_drawer_mask_styles)}></div>
                <div class="drawer" style={self.with_open_drawer_style(open_drawer_styles)}>
                    {
                        for TABS.iter().map(|tab| {
                            html! {
                                <AppRouterAnchor classes="drawer-tab-item__link" route={tab.route.clone()}>
                                    <div class="drawer-tab-item">
                                        {tab.name}
                                    </div>
                                </AppRouterAnchor>
                            }
                        })
                    }
                </div>
                <div class="container" >
                    <div class="header-wrapper">
                        <div class="left">
                            <div class="logo">
                                <img class="logo-icon" src=by_theme("/images/zzhack_icon_light.svg", "/images/zzhack_icon_dark.svg") />
                                <span class="zzhack-text">{"ZZHACK"}</span>
                            </div>
                            {
                                only_render_on_pc(html! {
                                    <div class="tabs">
                                        {
                                            for TABS.iter().map(|tab| {
                                                html! {
                                                    <AppRouterAnchor classes="tab-item" route={tab.route.clone()}>
                                                        {tab.name}
                                                    </AppRouterAnchor>
                                                }
                                            })
                                        }
                                    </div>
                                })
                            }
                        </div>
                        <div class="right">
                            <div onclick=self.link.callback(|_| HeaderMessage::SwitchTheme)>
                                <MatIconButton>
                                    <img src=by_theme("/images/dark_mode.svg", "/images/light_mode.svg") />
                                </MatIconButton>
                            </div>

                            {only_render_on_mobile(html! {
                                <img onclick=self.link.callback(|_| HeaderMessage::ToggleDrawer) class="menu" src=by_theme("/images/drawer_light.svg", "/images/drawer_dark.svg") />
                            })}
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
