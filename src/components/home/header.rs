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
use css_in_rust::style::Style;
use material_yew::{drawer::MatDrawerAppContent, MatDrawer, MatIconButton, MatTab, MatTabBar};
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
}

pub enum HeaderMessage {
    ChangeRoute(usize),
    SwitchTheme,
    Nope,
    ChangeTheme,
}

#[derive(Clone)]
pub struct Tab {
    pub route: AppRoutes,
    pub name: &'static str,
}

const GITHUB_PROFILE: &'static str = "https://github.com/youncccat";

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

            .title {
                font-weight: 500;
                color: var(--normal-text-color);
            }

            .tab_style {
                margin-left: 50px;
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
                        {"ZZHack"}
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
                </div>
            </div>
        }
    }
}
