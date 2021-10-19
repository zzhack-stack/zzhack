use crate::article_service;
use crate::console_log;
use crate::routes::app_routes::AppRoutes;
use crate::services::{
    theme_service::{DARK_THEME_KEY, LIGHT_THEME_KEY},
    ThemeService,
};
use crate::utils::theme::{only_render_on_mobile, only_render_on_pc};
use crate::workers::theme_agent::{ThemeAgent, ThemeAgentInput};
use crate::AppRouterAnchor;
use crate::Article;
use css_in_rust::style::Style;
use material_yew::MatButton;
use material_yew::{drawer::MatDrawerAppContent, MatDrawer, MatIconButton, MatTab, MatTabBar};
use regex::Regex;
use yew::prelude::*;
use yew_router::{
    agent::{RouteAgent, RouteRequest::ChangeRoute},
    route::Route,
    service::RouteService,
};

#[derive(Properties, Clone)]
pub struct HeaderProps {
    pub tabs: Vec<Tab>,
    pub on_menu_click: Callback<web_sys::MouseEvent>,
}

pub struct Header {
    route_service: RouteService<()>,
    props: HeaderProps,
    link: ComponentLink<Self>,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    style: Style,
    is_dark_theme: bool,
    theme_agent: Box<dyn Bridge<ThemeAgent>>,
    current_tab_index: u32,
    init_route: bool,
    search_result: String,
    search_articles: Vec<String>,
}

pub enum HeaderMessage {
    ChangeRoute(usize),
    SwitchTheme,
    Nope,
    ChangeTheme,
    UpdateSearchResult(String),
}

#[derive(Clone)]
pub struct Tab {
    pub route: AppRoutes,
    pub name: &'static str,
}

const GITHUB_PROFILE: &'static str = "https://github.com/mistricky";

fn get_github_oauth_url() -> String {
    let window = web_sys::window().unwrap();
    let origin = window.location().href().unwrap();

    format!(
        "https://github.com/login/oauth/authorize?client_id=20ac7165581dc3691b9d&redirect_uri=http://localhost:8080/oauth/redirect?origin={}",
        origin
    )

    // "https://github.com/login/oauth/authorize?client_id=20ac7165581dc3691b9d&redirect_uri=http://localhost:8080/oauth/redirect".to_string()
}

fn find_current_route_index(tabs: Vec<Tab>, current_route: Route) -> u32 {
    match tabs.iter().position(|tab| {
        let route: Route = tab.route.clone().into();
        current_route.contains(route.as_str())
    }) {
        Some(i) => i as u32,
        None => 0,
    }
}

impl Component for Header {
    type Message = HeaderMessage;
    type Properties = HeaderProps;

    fn create(props: HeaderProps, link: ComponentLink<Self>) -> Self {
        let route_service = RouteService::new();
        let route_agent = RouteAgent::bridge(link.callback(|_| HeaderMessage::Nope));
        let style = Style::create(
            "Header",
            "
            width: 100%;
            height: 75px;
            display: flex;
            justify-content: space-between;
            align-items: center;

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
            }

            .tabs {
                margin-left: 50px;
            }

            .menu {
                margin-left: 10px;
            }
        ",
        )
        .unwrap();

        let theme_service = ThemeService::new();
        let theme = theme_service.theme.clone();
        let theme_agent = ThemeAgent::bridge(link.callback(|_| HeaderMessage::ChangeTheme));
        let current_tab_index =
            find_current_route_index(props.tabs.clone(), route_service.get_route());

        Self {
            props,
            link,
            route_service,
            route_agent,
            style,
            is_dark_theme: theme == DARK_THEME_KEY,
            current_tab_index,
            theme_agent,
            init_route: false,
            search_result: String::from(""),
            search_articles: vec![],
        }
    }

    fn update(&mut self, msg: HeaderMessage) -> bool {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        html! {
            <div class="container">
                <div class=self.style.clone().to_string()>
                    <div class="left">
                        <div class="logo">
                            <img class="logo-icon" src="/images/zzhack_icon_light.svg" />
                            <span class="zzhack-text">{"ZZHACK"}</span>
                        </div>
                        {
                            only_render_on_pc(html! {
                                <div class="tabs">
                                    <AppRouterAnchor classes="tab-item" route={AppRoutes::Technology}>
                                        {"Technology"}
                                    </AppRouterAnchor>
                                    <AppRouterAnchor classes="tab-item" route={AppRoutes::Thinking}>
                                        {"Thinking"}
                                    </AppRouterAnchor>
                                    <AppRouterAnchor classes="tab-item" route={AppRoutes::Technology}>
                                        {"Fragments"}
                                    </AppRouterAnchor>
                                </div>
                            })
                        }
                    </div>

                    <div class="right">
                        <MatIconButton>
                            <img src="/images/dark_mode.svg" />
                        </MatIconButton>
                        {only_render_on_mobile(html! {<img class="menu" src="/images/drawer.svg" />})}
                    </div>
                </div>
            </div>
        }
    }
}
