mod app;
mod routes;
mod routes_switch;

extern crate lazy_static;
extern crate wee_alloc;

use app::App;
use yew::start_app;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    start_app::<App>();
}
