use crate::contact::{ContactType, Contacts};
use crate::container::Container;
use utils::use_style;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let style = use_style!(
        r"
        width: 100%;
        background: var(--base-color);
        padding-bottom: 18px;

        .contacts {
            margin-top: 31px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        .text {
            font-size: 14px;
        }

        @media (max-width: 600px) {
            .contacts {
                flex-direction: column;
                height: auto;
                padding-bottom: 30px;
            }
        }
    "
    );

    html! {
        <div class={style}>
            <Container>
                <div>
                    <div class="contacts">
                        <div>
                            <div class="text">{"🛠️ with Rust & Yew"}</div>
                        </div>
                        <div>
                            <Contacts source={vec![ContactType::Email, ContactType::GitHub, ContactType::Twitter, ContactType::LinkedIn,]} />
                        </div>
                    </div>
                </div>
            </Container>
        </div>
    }
}
