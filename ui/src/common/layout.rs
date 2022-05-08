use super::{footer::Footer, header::Header};
use crate::common::switch::ThemeSwitchBar;
use crate::container::Container;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
    pub children: Children,
}

#[function_component(BaseLayout)]
pub fn base_layout(props: &BaseLayoutProps) -> Html {
    html! {
        <>
            <Header/>
            <ThemeSwitchBar />
            <Container>
                { props.children.clone() }
            </Container>
            <Footer />
        </>
    }
}
