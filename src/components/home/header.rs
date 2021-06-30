    
use yew_router::{route::Route, service::RouteService, agent::{
    RouteAgent,
    RouteRequest::ChangeRoute,
}};
use material_yew:: {
    MatTabBar,
    MatTab,
    MatIconButton,
};
use yew::prelude::*;
use css_in_rust::style::Style;
use crate::routes::app_routes::{
    AppRoutes
};
use crate::services::{
    ThemeService,
    theme_service::{
        DARK_THEME_KEY,
    }
};
use crate::console_log;
use crate::utils::{
    theme::by_theme,
};

#[derive(Properties, Clone)]
pub struct HeaderProps {}

pub struct Header {
    route_service: RouteService<()>,
    theme_service: ThemeService,
    props: HeaderProps,
    link: ComponentLink<Self>,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    style: Style,
    is_dark_theme: bool
}

pub enum HeaderMessage {
    ChangeRoute(Route<()>),
    SwitchTheme
}

struct Tab {
    route: AppRoutes,
    name: &'static str
}

const GITHUB_PROFILE: &'static str = "https://github.com/youncccat";

const TABS:[Tab; 3] = [
    Tab {route: AppRoutes::Technology, name: "技术"},
    Tab {route: AppRoutes::Thinking, name: "随想"},
    Tab {route: AppRoutes::AboutMe, name: "关于我"},
];

impl Component for Header {
    type Message = HeaderMessage;
    type Properties = HeaderProps;

    fn create(props: HeaderProps, link: ComponentLink<Self>) -> Self {
        let route_service = RouteService::new();
        let route_agent = RouteAgent::bridge(link.callback(HeaderMessage::ChangeRoute));
        let style = Style::create("Header", r#"
            height: 48px;
            width: 100%;
            display: flex;
            align-items: center;
            justify-content: space-between;
            position: relative;
            z-index: 1;
            background: var(--base-color); 
            box-shadow: 0 1px 1px 0 rgb(32 33 36 / 28%);
            padding: 0 24px;
            box-sizing: border-box;

            .left {
                display: flex;
                align-items: center;
            }

            .right {
                display: flex;
                align-items: center;
            }

            .title {
                font-weight: 500;
                color: var(--normal-text-color);
            }

            .tab_style {
                margin-left: 50px;
            }
        "#).unwrap();
        
        let theme_service = ThemeService::new();
        let theme = theme_service.theme.clone();

        Self {
            props,
            link,
            route_service,
            route_agent,
            style,
            theme_service,
            is_dark_theme: theme == DARK_THEME_KEY,
        } 
    }

    fn update(&mut self, msg: HeaderMessage) -> bool { 
        match msg {
            HeaderMessage::ChangeRoute(route) => {
                let current_route = self.route_service.get_route();

                console_log!("{} {} hhashhashsadhhash", current_route, route);

                if current_route == route {
                    return false                
                }

                self.route_agent.send(ChangeRoute(route));
            },
            HeaderMessage::SwitchTheme => {
                let target_theme = !self.is_dark_theme;

                self.theme_service.switch(target_theme);
                self.is_dark_theme = target_theme;
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }


    fn view(&self) -> yew::virtual_dom::VNode { 
        html! {
                <div class=self.style.clone().to_string()>
                    <div class="left">
                        <div class="title">
                            {"Mist's Blog"}
                        </div>
                        <div class="tab_style">
                            <MatTabBar onactivated=self.link.callback(|i: usize| HeaderMessage::ChangeRoute(TABS[i].route.clone().into()))>
                                {for TABS.iter().map(|tab| html!{
                                    <MatTab label=tab.name is_fading_indicator=true />
                                })}
                            </MatTabBar>
                        </div>
                    </div>
                    <div class="right">  
                        <div onclick=self.link.callback(|_| HeaderMessage::SwitchTheme)>
                            <MatIconButton>
                                <img src=by_theme("/images/dark_mode.svg", "/images/light_mode.svg") />
                            </MatIconButton>
                        </div>
                        <a href=GITHUB_PROFILE>
                            <MatIconButton>
                                <img src=by_theme("/images/github_dark.svg", "/images/github_light.svg") />
                            </MatIconButton>
                        </a>
                    </div>
                </div>
        }
    }
}