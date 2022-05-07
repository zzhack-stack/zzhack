use super::{footer::Footer, header::Header};
use crate::common::switch::ThemeSwitchBar;
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
            <ThemeSwitchBar />
            <div classes={style}>
                { props.children.clone() }
            </div>
            <Footer />
        </>
    }
}
