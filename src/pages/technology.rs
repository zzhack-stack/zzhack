use crate::components::home::category_bar::CategoryBar;
use yew::prelude::*;

pub struct Technology;

impl Component for Technology {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <CategoryBar text="Technology is life" categories=vec!("文章", "开源") />
            </div>
        }
    }
}