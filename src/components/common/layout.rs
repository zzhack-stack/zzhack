use super::{footer::Footer, header::Header};
use crate::components::common::switch::ThemeSwitch;
use crate::services::theme_service::ThemeService;
use crate::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
    pub children: Children,
}

#[function_component(BaseLayout)]
pub fn base_layout(props: &BaseLayoutProps) -> Html {
    let style = use_style!(r#""#);

    html! {
        <>
            <Header/>
            <ThemeSwitch default={ThemeService::from_storage().get_theme().clone()} />
            <div classes={style}>
                { props.children.clone() }
            </div>
            <Footer />
        </>
    }
}
