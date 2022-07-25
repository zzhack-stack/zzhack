use super::{footer_source::CONTACTS, image::Icon};
use crate::link::Link;
use utils::use_style;
use yew::prelude::*;

#[function_component(Contacts)]
pub fn contacts() -> Html {
    let style = use_style!(
        r"
        display: flex;
        align-items: center;
    "
    );

    let render_contacts = CONTACTS
        .iter()
        .map(|contact| {
            html! {
                <Link out_href={contact.link}>
                    <Icon source={contact.icon} size={contact.icon_size} has_theme={false} is_raw_source=true />
                </Link>
            }
        })
        .collect::<Html>();

    html! {
        <div class={style}>
            {render_contacts}
        </div>
    }
}
