mod pages;
mod components;
mod routes;

use crate::routes::switch;
use yew_router::prelude::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use pages::{
    home::Home,
    about_me::AboutMe,
    not_found::NotFound,
};
use routes::Routes;
use components::Header;
use material_yew::{
    MatTopAppBar,
    MatTopAppBarFixed,
    top_app_bar_fixed::{MatTopAppBarNavigationIcon},
    MatIconButton
};

struct Root {}

pub struct Msg {}

type AppRouter = Router<Routes>;

impl Component for Root {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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
                <Header text="Mist's Blog"></Header>
                <AppRouter render = Router::render(switch) />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
