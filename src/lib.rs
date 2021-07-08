mod agents;
mod components;
mod pages;
mod routes;
mod services;
mod utils;
use crate::components::home::header::Tab;
use crate::routes::app_routes::switch;
use crate::routes::app_routes::AppRouterAnchor;
use crate::services::ThemeService;
use components::{Footer, Header};
use material_yew::drawer::{MatDrawer, MatDrawerAppContent};
use material_yew::MatList;
use material_yew::MatListItem;
use routes::app_routes::AppRoutes;
use stdweb::js;
use stdweb::Value::Number;
use wasm_bindgen::prelude::*;
use yew::agent::Context;
use yew::prelude::*;
use yew_router::prelude::*;

struct Root {
    link: ComponentLink<Self>,
    is_open_drawer: bool,
    tabs: Vec<Tab>,
    theme_service: ThemeService,
}

pub enum RootMessage {
    ToggleDrawer(bool),
    SwitchDrawer,
}

type AppRouter = Router<AppRoutes>;

impl Component for Root {
    type Message = RootMessage;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tabs = vec![
            Tab {
                route: AppRoutes::Technology,
                name: "技术",
            },
            Tab {
                route: AppRoutes::Thinking,
                name: "随想",
            },
            Tab {
                route: AppRoutes::AboutMe,
                name: "关于我",
            },
        ];

        ThemeService::init();
        let theme_service = ThemeService::new();

        Self {
            link,
            is_open_drawer: false,
            tabs,
            theme_service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMessage::ToggleDrawer(is_open_drawer) => {
                self.is_open_drawer = is_open_drawer;
                return false;
            }
            RootMessage::SwitchDrawer => self.is_open_drawer = !self.is_open_drawer,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // ContextProvider::a();
        true
    }

    fn view(&self) -> Html {
        html! {
            <MatDrawer
                drawer_type="dismissible"
                open=self.is_open_drawer
                onopened=self.link.callback(|_| RootMessage::ToggleDrawer(true))
                onclosed=self.link.callback(|_| RootMessage::ToggleDrawer(false))
            >
                <div style="background: var(--side-bar-color); height: 100%;">
                    <MatList>
                        {for self.tabs.iter().map(|tab| html!{
                            <AppRouterAnchor route=tab.route.clone()>
                                <div onclick=self.link.callback(|_| RootMessage::SwitchDrawer)>
                                    <MatListItem>
                                        <span class="text">{tab.name}</span>
                                    </MatListItem>
                                </div>
                            </AppRouterAnchor>
                        })}
                    </MatList>
                </div>
                <MatDrawerAppContent>
                    <Header
                        tabs=self.tabs.clone()
                        on_menu_click=self.link.callback(|_| RootMessage::SwitchDrawer)
                    />
                    // <ContextProvider></ContextProvider>
                    <AppRouter render = Router::render(switch) />
                    <Footer />
                </MatDrawerAppContent>
            </MatDrawer>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
