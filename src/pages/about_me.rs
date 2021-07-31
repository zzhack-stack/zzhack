use crate::components::home::category_bar::{Category, CategoryBar};
use crate::routes::about_routes::{switch, AboutRouter, AboutRoutes};
use css_in_rust::Style;
use yew::prelude::*;
use yew_router::prelude::Router;

pub struct AboutMe {
    style: Style,
}

pub enum AboutMeMessage {}

impl Component for AboutMe {
    type Message = AboutMeMessage;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "AboutMe",
            r#"
            min-height: calc(100% - 48px);
        "#,
        )
        .unwrap();

        Self { style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <CategoryBar dark_icon="/images/help_dark.svg" light_icon="/images/help_light.svg" text="帮助中心" categories=vec!(Category {name: "关于我", route: AboutRoutes::AboutMe.into()}, Category {name: "帮助", route: AboutRoutes::AboutHelp.into()}) />
                <AboutRouter render=Router::render(switch) />
            </div>
        }
    }
}
