use crate::console_log;
use crate::utils::theme::by_theme;
use crate::workers::theme_agent::ThemeAgent;
use css_in_rust::style::Style;
use material_yew::{MatTab, MatTabBar};
use yew::prelude::*;
use yew_router::{
    agent::{RouteAgent, RouteRequest::ChangeRoute},
    route::Route,
    service::RouteService,
};

#[derive(Clone)]
pub struct Category {
    pub name: &'static str,
    pub route: Route,
}

#[derive(Properties, Clone)]
pub struct CategoryBarProps {
    pub categories: Vec<Category>,
    pub text: &'static str,
    pub light_icon: &'static str,
    pub dark_icon: &'static str,
}

pub struct CategoryBar {
    props: CategoryBarProps,
    link: ComponentLink<CategoryBar>,
    style: Style,
    route_service: RouteService,
    route_agent: Box<dyn Bridge<RouteAgent>>,
    theme_agent: Box<dyn Bridge<ThemeAgent>>,
}

pub enum CategoryBarMessage {
    ChangeRoute(usize),
    ChangeTheme,
    Nope,
}

impl Component for CategoryBar {
    type Message = CategoryBarMessage;
    type Properties = CategoryBarProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let route_agent = RouteAgent::bridge(link.callback(|_| CategoryBarMessage::Nope));
        let route_service = RouteService::new();
        let style = Style::create(
            "CategoryBar",
            r#"
            width: 100%;
            height: 118px;
            background: var(--category-color);

            .text {
                font-family: 'Roboto';
                font-size: 16px;
                height: 60px;
                display: flex;
                align-items: center;
                padding: 10px 24px 0 24px;
                color: #636e72;
            }

            .tabs {
                height: 48px;
                display: flex;
            }

            .icon {
                margin-right: 10px;
            }
        "#,
        )
        .unwrap();
        let theme_agent = ThemeAgent::bridge(link.callback(|_| CategoryBarMessage::ChangeTheme));

        Self {
            props,
            link,
            style,
            route_agent,
            route_service,
            theme_agent,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            CategoryBarMessage::Nope => return false,
            CategoryBarMessage::ChangeRoute(i) => {
                let route = self.props.categories[i].route.clone();
                let current_route = self.route_service.get_route();

                if current_route == route {
                    return false;
                }

                self.route_agent.send(ChangeRoute(route));
            }
            CategoryBarMessage::ChangeTheme => {
                console_log!("hello world");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="text">
                    <img class="icon" src={by_theme(self.props.dark_icon, self.props.light_icon)} />
                    {self.props.text}
                </div>
                <div class="tabs">
                    <MatTabBar onactivated=self.link.callback(|i: usize| CategoryBarMessage::ChangeRoute(i))>
                        {for self.props.categories.iter().map(|category| html!{
                            <MatTab label=category.name.clone() is_min_width_indicator=true min_width=true is_fading_indicator=true />
                        })}
                    </MatTabBar>
                </div>
            </div>
        }
    }
}
