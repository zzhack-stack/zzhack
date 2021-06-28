    
    use yew_router::{route::Route, service::RouteService, Switch, agent::{
    RouteAgent,
    RouteRequest::ChangeRoute,
}};
use material_yew::MatTab;
use material_yew::MatTabBar;
use yew::prelude::*;
use crate::routes::{
    AppRouterAnchor,
    Routes
};

pub struct Header {
    route_service: RouteService<()>,
    props: HeaderProps,
    link: ComponentLink<Self>,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
}

pub enum HeaderMessage {
    ChangeRoute(Route<()>)
}

#[derive(Debug, Properties, Clone)]
pub struct HeaderProps {
    pub text: String,
}

struct Tab {
    route: Routes,
    name: &'static str
}

const TABS:[Tab; 3] = [
    Tab {route: Routes::Technology, name: "技术"},
    Tab {route: Routes::Thinking, name: "随想"},
    Tab {route: Routes::AboutMe, name: "关于我"},
];

impl Component for Header {
    type Message = HeaderMessage;
    type Properties = HeaderProps;

    fn create(props: HeaderProps, link: ComponentLink<Self>) -> Self {
        let mut route_service = RouteService::new();
        let mut route_agent = RouteAgent::bridge(link.callback(HeaderMessage::ChangeRoute));
        route_agent.send(ChangeRoute(Routes::NotFound.into()));

        Self {
            props,
            link,
            route_service,
            route_agent,
        } 
    }

    fn update(&mut self, msg: HeaderMessage) -> bool { 
        match msg {
            HeaderMessage::ChangeRoute(route) => {
                let current_route = self.route_service.get_route();

                if current_route == route {
                    return false                
                }

                self.route_agent.send(ChangeRoute(route));
            },
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }


    fn view(&self) -> yew::virtual_dom::VNode { 
        let header_wrapper_style = "
            height: 48px;
            width: 100%;
            display: flex;
            align-items: center;
            background: var(--base-color); 
            box-shadow: 0 1px 6px 0 rgb(32 33 36 / 28%);
            padding: 0 24px;
        ";

        let title_style = "
            font-weight: 500;
        ";

        let tab_style = "
            margin-left: 50px;
        ";

        html! {
            <div style=header_wrapper_style>
                <div style=title_style>
                    {self.props.text.clone()}
                </div>
                <div style=tab_style>
                    <MatTabBar onactivated=self.link.callback(|i: usize| HeaderMessage::ChangeRoute(TABS[i].route.clone().into()))>
                        {for TABS.iter().map(|tab| html!{
                            <MatTab label=tab.name is_fading_indicator=true />
                        })}
                    </MatTabBar>
                </div>
            </div>
        }
    }
}