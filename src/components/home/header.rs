    
use yew_router::{route::Route, service::RouteService, agent::{
    RouteAgent,
    RouteRequest::ChangeRoute,
}};
use material_yew::MatTab;
use material_yew::MatTabBar;
use yew::prelude::*;
use css_in_rust::style::Style;
use crate::routes::{
    Routes
};

#[derive(Properties, Clone)]
pub struct HeaderProps {

}

pub struct Header {
    route_service: RouteService<()>,
    props: HeaderProps,
    link: ComponentLink<Self>,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    style: Style
}

pub enum HeaderMessage {
    ChangeRoute(Route<()>)
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
        let route_service = RouteService::new();
        let route_agent = RouteAgent::bridge(link.callback(HeaderMessage::ChangeRoute));
        let style = Style::create("Header", r#"
            height: 48px;
            width: 100%;
            display: flex;
            align-items: center;
            position: relative;
            z-index: 1;
            background: var(--base-color); 
            box-shadow: 0 1px 1px 0 rgb(32 33 36 / 28%);
            padding: 0 24px;
            box-sizing: border-box;

            .title {
                font-weight: 500;
            }

            .tab_style {
                margin-left: 50px;
            }
        "#).unwrap();

        Self {
            props,
            link,
            route_service,
            route_agent,
            style,
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
        html! {
            <>
                <div class=self.style.clone().to_string()>
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
            </>
        }
    }
}