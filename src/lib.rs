mod utils;
mod pages;
mod components;
mod routes;
mod services;

use crate::services::ThemeService;
use crate::routes::app_routes::switch;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use routes::app_routes::AppRoutes;
use components::{
    Header,
    Footer,
};

struct Root {}

pub struct Msg {}

type AppRouter = Router<AppRoutes>;

impl Component for Root {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        ThemeService::init();

        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header />
                <AppRouter render = Router::render(switch) />
                <Footer />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
