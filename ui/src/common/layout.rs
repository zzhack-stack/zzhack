use super::{footer::Footer, header::Header};
use crate::common::switch::ThemeSwitchBar;
use crate::container::Container;
use utils::use_style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
    pub children: Children,
}

#[function_component(BaseLayout)]
pub fn base_layout(props: &BaseLayoutProps) -> Html {
    let style = use_style!(
        r"
        width: 100%;
        height: 100%;
        position: relative;

        .theme-switch-bar {
            position: absolute;
            right: -118px;
            top: 63px;
        }

        .page-outlet {
            min-height: calc(100vh - 209px);
        }

        @media (max-width: 600px) {
            .theme-switch-bar {
                position: static;
            }
        }
    "
    );

    html! {
        <>
            <Header/>
            <Container>
                <div class={style}>
                    <div class="theme-switch-bar">
                        <ThemeSwitchBar />
                    </div>
                    <div class="page-outlet">
                        { props.children.clone() }
                    </div>
                </div>
            </Container>
            <Footer />
        </>
    }
}
