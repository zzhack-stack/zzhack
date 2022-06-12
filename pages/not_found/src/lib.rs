use stylist::style;
use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let style = style!(
        r"
        padding-top: 100px;
        display: flex;
        flex-direction: column;
        align-items: center;

        & > img {
            width: 400px;
        }

        @media (max-width: 600px) {
            padding-top: 50px;

            & > img {
                width: 100%;
            }
        }
    "
    )
    .unwrap();

    html! {
        <div class={style}>
            <img src="/images/page_not_found.png" />
        </div>
    }
}
