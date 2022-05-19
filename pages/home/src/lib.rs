use ui::image::ThemeImage;
use utils::use_style;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let style = use_style!(
        r"
        .banner {
            display: flex;
            justify-content: center;
            margin-top: 63px;
        }

        @media (max-width: 600px) {
            .banner {
                width: 100%;
                margin-top: 32px;
            }
        }
    "
    );

    html! {
        <div class={style}>
            <div class="banner">
                <ThemeImage source="banner.svg" is_reactive=true />
            </div>
        </div>
    }
}
