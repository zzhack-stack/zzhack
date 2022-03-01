use css_in_rust::Style;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::Component;

#[derive(Clone)]
pub enum PopupTrigger {
    Hover,
    Click,
}

pub enum PopupMessage {
    TriggerPopup,
}

#[derive(Properties, Clone)]
pub struct PopupProps {
    pub bind: VNode,
    pub trigger: PopupTrigger,
    pub offset: (i32, i32),
    pub children: Children,
    pub has_default_padding: bool,
}

pub struct Popup {
    style: Style,
    props: PopupProps,
    link: ComponentLink<Popup>,
    is_open_popup: bool,
}

impl Component for Popup {
    type Message = PopupMessage;
    type Properties = PopupProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Popup",
            r#"
                position: relative;

                .popup-body {
                    position: absolute;
                    border-radius: 5px;
                    box-shadow: rgb(100 100 111 / 20%) 0px 7px 29px 0px;
                    transition: 0.3s all;
                    background: var(--base-color);
                    padding: 15px;
                    z-index: 5;
                    overflow: hidden;
                } 

                .popup-item {
                    cursor: pointer;
                }

                .popup-mask {
                    position: fixed;
                    width: 100vw;
                    height: 100vh;
                    left: 0;
                    top: 0;
                    z-index: 3;
                }

                .close-btn {
                    display: none;       
                }

                @media (max-width: 600px) {
                    .popup-body {
                        position: fixed;
                        left: 0;
                        top: 0;
                        width: 100%;
                        height: 100vh;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        background: var(--deep-mask-color);
                        transition: 0.3s opacity;
                    }

                    .close-btn {
                        display: block;
                        position: absolute;
                        top: 30px;
                        left: 30px;
                        width: 25px;
                        height: 25px;
                        padding: 10px;
                        border-radius: 50%;
                        background: white;
                        cursor: pointer;
                    }
                }
            "#,
        )
        .unwrap();

        Self {
            style,
            props,
            link,
            is_open_popup: false,
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            PopupMessage::TriggerPopup => {
                self.is_open_popup = !self.is_open_popup;

                true
            }
        }
    }

    fn view(&self) -> Html {
        let bind = &self.props.bind;
        let offset = self.props.offset;
        let animation_factor = if self.is_open_popup { 1 } else { 0 };
        let popup_body_styles = format!(
            "transform: scale({}); transform-origin: {}px {}px; padding: {}px; opacity: {};",
            animation_factor,
            offset.0,
            offset.1,
            if self.props.has_default_padding {
                15
            } else {
                0
            },
            animation_factor
        );
        let popup_mask_styles = format!(
            "display: {}",
            if self.is_open_popup { "block" } else { "none" }
        );

        html! {
            <div class=self.style.to_string()>
                <div class="popup-mask" onclick=self.link.callback(|_| PopupMessage::TriggerPopup) style=popup_mask_styles></div>
                <div class="popup-item" onclick=self.link.callback(|_| PopupMessage::TriggerPopup)>
                    {bind.clone()}
                </div>
                <div class="popup-body" style=popup_body_styles>
                    <div class="close-btn" onclick=self.link.callback(|_| PopupMessage::TriggerPopup)>
                        <img src="/images/close.svg"/>
                    </div>
                    {self.props.children.clone()}
                </div>
            </div>
        }
    }
}
