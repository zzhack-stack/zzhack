use crate::article_service;
use crate::console_log;
use crate::routes::app_routes::AppRoutes;
use crate::services::{
    theme_service::{DARK_THEME_KEY, LIGHT_THEME_KEY},
    ThemeService,
};
use crate::utils::theme::by_reactive;
use crate::utils::theme::by_theme;
use crate::utils::theme::is_on_mobile;
use crate::workers::theme_agent::{ThemeAgent, ThemeAgentInput};
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

const GITHUB_PROFILE: &'static str = "https://github.com/youncccat";

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
            height: 48px;
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: space-between;
            position: relative;
            z-index: 11;
            background: var(--base-color); 
            box-shadow: 0 1px 1px 0 rgb(32 33 36 / 28%);
            padding: 0 24px;
            box-sizing: border-box;
            position: sticky;
            top: 0;

            .left {
                display: flex;
                align-items: center;
                flex-shrink: 0;
            }

            .right {
                display: flex;
                align-items: center;
                flex-shrink: 0;
            }

            .zzhack_icon {
                width: 30px;
            }

            .zzhack {
                height: 16px;
                margin-left: 10px;
            }

            .title {
                display: flex;   
                align-items: center;             
            }

            .tab_style {
                margin-left: 25px;
            }

            .search-wrapper {
                height: 30px;
                border-radius: 5px;
                background: var(--search-bg-color);
                padding: 0 5px;
                display: flex;
                position: relative;
                border: solid 1px transparent;
            }

            .search-input {
                width: 179px;
                background: transparent;
                border: none;
                outline: none;
                font-size: 14px;
                transition: 0.3s all;
                color: var(--normal-text-color);
            }

            .search-input:focus ~.search-result {
                transform: translateX(0);
                opacity: 1;
            }

            .search-wrapper:focus {
                border-color: #fdcb6e;
            }

            .search-icon {
                width: 18px;
            }

            .search-result {
                width: 300px;
                min-height: 150px;
                background: var(--primary-color);
                position: absolute;
                right: 0;
                top: 100%;
                border-radius: 5px;
                transform: translateX(140%);
                opacity: 0;
                transition: 0.3s all;
                margin-top: 20px;
                color: var(--search-card-color);
                padding: 10px 0;
            }

            .search-result-text {
                color: var(--search-card-color);
            }

            .search-result-wrapper {
                width: 100%;
                height: 150px;
                display: flex;
                justify-content: center;
                align-items: center;
            }

            .search-article {
                width: 100%;
                height: 48px;
                transition: 0.3s all;
                color: var(--search-card-color);
                border-bottom: 1px solid var(--search-card-hover-color);
                display: flex;
                align-items: center;
                font-size: 14px;
                padding: 0 10px;
            }

            .search-article:hover {
                background: var(--search-card-hover-color);
            }

            .search-result-box {
                width: 100%;
            }

            @media (max-width: 600px){
                .search-wrapper {
                    display: none;
                }
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
        match msg {
            HeaderMessage::ChangeRoute(i) => {
                let route: Route = self.props.tabs[i].route.clone().into();
                let current_route = self.route_service.get_path();

                if !self.init_route {
                    self.init_route = true;
                    return false;
                }

                if current_route == route.to_string()
                    || current_route.starts_with(route.to_string().as_str())
                {
                    return false;
                }

                self.route_agent.send(ChangeRoute(route));
            }
            HeaderMessage::SwitchTheme => {
                let target_theme = !self.is_dark_theme;

                self.is_dark_theme = target_theme;
                self.theme_agent
                    .send(ThemeAgentInput::ChangeTheme(if self.is_dark_theme {
                        DARK_THEME_KEY
                    } else {
                        LIGHT_THEME_KEY
                    }));
            }
            HeaderMessage::ChangeTheme => {}
            HeaderMessage::UpdateSearchResult(input) => {
                self.search_result = input;

                if &self.search_result == "" {
                    self.search_articles = vec![];
                    return true;
                }

                let title_regex = match Regex::new(&self.search_result) {
                    Ok(regex) => regex,
                    Err(_) => return false,
                };

                let articles = unsafe { article_service.get_articles_by_pattern(title_regex) };

                self.search_articles = articles
                    .iter()
                    .map(|article| article.title.clone())
                    .collect();
            }
            HeaderMessage::Nope => return false,
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render && is_on_mobile() {
            self.link
                .send_message(HeaderMessage::ChangeRoute(self.current_tab_index as usize))
        }
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        html! {
            <div class=self.style.clone().to_string()>
                <div class="left">
                    {by_reactive(html! {<div onclick=&self.props.on_menu_click style=format!("color: {}", by_theme("black", "white"
                        ))>
                        <MatIconButton icon="menu" />
                    </div>}, html!{})}
                    <div class="title">
                        <img class="zzhack_icon" src=by_theme("/images/zzhack_icon_dark.svg", "/images/zzhack_icon_light.svg") />
                        <img class="zzhack" src=by_theme("/images/zzhack_dark.svg", "/images/zzhack_light.svg") />
                    </div>
                    {by_reactive(html!{}, html!{<div class="tab_style">
                        <MatTabBar active_index=self.current_tab_index onactivated=self.link.callback(|i: usize| HeaderMessage::ChangeRoute(i))>
                            {for self.props.tabs.iter().map(|tab| html!{
                            <MatTab label=tab.name is_fading_indicator=true />
                            })}
                        </MatTabBar>
                    </div>})}
                </div>
                <div class="right">
                    <div class="search-wrapper">
                        <input class="search-input" oninput=self.link.callback(|e:InputData| HeaderMessage::UpdateSearchResult(e.value)) />
                        <img class="search-icon" src=by_theme("/images/search_dark.svg", "/images/search_light.svg") />
                        <div class="search-result">
                            {
                                if self.search_articles.len() == 0 {
                                    html! {
                                        <div class="search-result-wrapper">
                                            <span class="search-result-text">{"Nothing."}</span>
                                        </div>
                                    }
                                } else {
                                    html!{
                                        <div class="search-result-box">
                                            <div class="search-article">
                                                {format!("找到以下 {} 个结果：", self.search_articles.len())}
                                            </div>
                                            {
                                                for self.search_articles.iter().map(|title| {
                                                    html!{ <div class="search-article">{title}</div>}
                                                })
                                            }
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>
                    <div onclick=self.link.callback(|_| HeaderMessage::SwitchTheme)>
                        <MatIconButton>
                            <img src=by_theme("/images/dark_mode.svg", "/images/light_mode.svg" ) />
                        </MatIconButton>
                    </div>
                    <a href=GITHUB_PROFILE>
                        <MatIconButton>
                            <img src=by_theme("/images/github_dark.svg", "/images/github_light.svg" ) />
                        </MatIconButton>
                    </a>
                    <a href=get_github_oauth_url()>
                        <MatButton  label="登录" />
                    </a>
                </div>
            </div>
        }
    }
}
