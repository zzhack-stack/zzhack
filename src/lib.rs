mod components;
mod pages;
mod routes;
mod services;
mod store;
mod utils;
mod workers;
use crate::components::common::Snackbar;
use crate::components::home::header::Tab;
use crate::routes::app_routes::switch;
use crate::routes::app_routes::AppRouterAnchor;
use crate::services::api_service::Res;
use crate::services::article_service::article_service;
use crate::services::article_service::Article;
use crate::services::provider_service::provider_service;
use crate::services::provider_service::RootMetadata;
use crate::services::CacheService;
use crate::services::ThemeService;
use crate::store::StoreStates;
use components::{Footer, Header};
use css_in_rust::Style;
use material_yew::drawer::{MatDrawer, MatDrawerAppContent};
use material_yew::MatList;
use material_yew::MatListItem;
use routes::app_routes::AppRoutes;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
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
    root_metadata_fetch_task: FetchTask,
}

pub enum RootMessage {
    ToggleDrawer(bool),
    SwitchDrawer,
    StoreState(Rc<StoreStates>),
    SyncArticles(Vec<Article>),
    UpdateRootMetadata(RootMetadata),
}

type AppRouter = Router<AppRoutes>;

impl Component for Root {
    type Message = RootMessage;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let tabs = vec![
            Tab {
                route: AppRoutes::Technology,
                name: "文章",
            },
            Tab {
                route: AppRoutes::Thinking,
                name: "随想",
            },
            Tab {
                route: AppRoutes::About,
                name: "关于",
            },
        ];

        ThemeService::init();
        let theme_service = ThemeService::new();
        let style = Style::create(
            "Root",
            r#"
            height: 100%;
            width: 100%;
            display: flex;
            justify-content: center;
            align-items: center;
            font-size: 33px;
        "#,
        )
        .unwrap();
        let dispatch = Dispatch::bridge_state(link.callback(RootMessage::StoreState));
        let root_metadata_fetch_task = provider_service
            .get_root_metadata(link.callback(|metadata| RootMessage::UpdateRootMetadata(metadata)));
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
            root_metadata_fetch_task,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            // unsafe {
            //     self.task = Some(article_service.sync_articles(self.link.callback(
            //         |response: Res<QueryRes<Article>>| {
            //             let Json(data) = response.into_body();
            //             let articles = data.unwrap().items.clone();
            //             RootMessage::SyncArticles(articles)
            //         },
            //     )));
            // }
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
            RootMessage::UpdateRootMetadata(metadata) => {
                CacheService::new().set_root_metadata(metadata);
                self.is_sync_data = true;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                {"🚧 Refactoring is underway."}
            </div>
        }
    }
}

impl Root {
    fn render_loading(&self) -> Html {
        html! {
            <div class="loading-wrapper">
                <img class="loading-icon" src="/images/zzhack_icon_light.svg" />
                // <img class="loading-text" src="/images/zzhack_light.svg" />
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
                    <Snackbar />
                    <AppRouter redirect=Router::redirect(|_| AppRoutes::Technology) render = Router::render(switch) />
                </MatDrawerAppContent>
            </MatDrawer>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
