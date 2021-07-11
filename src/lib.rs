mod agents;
mod components;
mod pages;
mod routes;
mod services;
mod store;
mod utils;
use crate::components::home::header::Tab;
use crate::routes::app_routes::switch;
use crate::routes::app_routes::AppRouterAnchor;
use crate::services::api_service::Res;
use crate::services::article_service::article_service;
use crate::services::article_service::Article;
use crate::services::article_service::QueryRes;
use crate::services::ArticleService;
use crate::services::ThemeService;
use crate::store::StoreStates;
use components::{Footer, Header};
use css_in_rust::Style;
use material_yew::drawer::{MatDrawer, MatDrawerAppContent};
use material_yew::MatList;
use material_yew::MatListItem;
use routes::app_routes::AppRoutes;
use std::rc::Rc;
use stdweb::js;
use stdweb::Value::Number;
use wasm_bindgen::prelude::*;
use yew::agent::Context;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew_router::prelude::*;
use yewdux::prelude::BasicStore;
use yewdux::prelude::Dispatch;

struct Root {
    link: ComponentLink<Self>,
    is_open_drawer: bool,
    tabs: Vec<Tab>,
    theme_service: ThemeService,
    is_sync_data: bool,
    style: Style,
    state: Rc<StoreStates>,
    dispatch: Dispatch<BasicStore<StoreStates>>,
    task: Option<FetchTask>,
}

pub enum RootMessage {
    ToggleDrawer(bool),
    SwitchDrawer,
    StoreState(Rc<StoreStates>),
    SyncArticles(Vec<Article>),
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
                route: AppRoutes::Books(2),
                name: "关于我",
            },
        ];

        ThemeService::init();
        let theme_service = ThemeService::new();
        let style = Style::create(
            "Root",
            r#"
            height: 100%;

            .loading-wrapper {
                height: 100%;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            }
        "#,
        )
        .unwrap();
        let dispatch = Dispatch::bridge_state(link.callback(RootMessage::StoreState));

        Self {
            link,
            is_open_drawer: false,
            tabs,
            theme_service,
            style,
            is_sync_data: false,
            state: Default::default(),
            dispatch,
            task: None,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            unsafe {
                self.task = Some(article_service.sync_articles(self.link.callback(
                    |response: Res<QueryRes<Article>>| {
                        let Json(data) = response.into_body();
                        let articles = data.unwrap().items.clone();
                        RootMessage::SyncArticles(articles)
                    },
                )));
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            RootMessage::ToggleDrawer(is_open_drawer) => {
                self.is_open_drawer = is_open_drawer;
                return false;
            }
            RootMessage::SwitchDrawer => self.is_open_drawer = !self.is_open_drawer,
            RootMessage::StoreState(state) => self.state = state,
            RootMessage::SyncArticles(articles) => {
                unsafe {
                    article_service.set_articles(articles);
                }
                self.is_sync_data = true
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        // ContextProvider::a();
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                {
                    if self.is_sync_data {
                        self.render_root()
                    } else {
                        self.render_loading()
                    }
                }
            </div>
        }
    }
}

impl Root {
    fn render_loading(&self) -> Html {
        html! {
            <div class="loading-wrapper">
                <img src="https://img-blog.csdnimg.cn/20210709181729301.gif" />
                <p class="text">{"正在探索新大陆......"}</p>
            </div>
        }
    }

    fn render_root(&self) -> Html {
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
