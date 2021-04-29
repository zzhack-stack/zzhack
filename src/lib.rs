use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Root {}

pub struct Msg {}

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
        false
    }

    fn view(&self) -> Html {
        let wrapper_style =
            "display: flex; justify-content: center; align-items: center; width: 100%; height: 100%;";

        html! {
            <div style=wrapper_style>
                <div>
                    <p>{"Hiiiii, I'm ZhanHao Zhao, u can just call me Mist."}</p>
                    <p>{"There is my personal blog site, but there is nothing here for the time being, in the future, there may be some interesting or boring things here which come from my life❤️."}</p>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
