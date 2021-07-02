use css_in_rust::Style;
use yew_router::prelude::Router;
use crate::components::home::category_bar::{
    CategoryBar,
    Category,
};
use yew::prelude::*;
use crate::routes::technology_routes::{
    TechnologyRoutes,
    switch,
    TechnologyRouter
};

pub struct Technology {
    style: Style
}

impl Component for Technology {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create("Technology", r#"
            min-height: calc(100% - 48px);
        "#).unwrap();

        Self {
            style,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <CategoryBar text="Technology is life" categories=vec!(Category {name: "文章", route: TechnologyRoutes::Articles.into()}, Category {name: "开源", route: TechnologyRoutes::OpenSource.into()}) />
                <TechnologyRouter render=Router::render(switch) />
            </div>
        }
    }
}