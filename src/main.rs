mod app;
mod components;
mod global;
mod pages;
mod routes;
mod services;
#[macro_use]
mod utils;

#[macro_use]
extern crate lazy_static;

use app::App;
use yew::start_app;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    start_app::<App>();
}
