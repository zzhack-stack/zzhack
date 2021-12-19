use crate::workers::snackbar_agent::SnackbarAgent;
use css_in_rust::Style;
use material_yew::MatButton;
use material_yew::MatSnackbar;
use material_yew::WeakComponentLink;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct SnackbarProps {}

pub struct Snackbar {
    style: Style,
    props: SnackbarProps,
    link: ComponentLink<Snackbar>,
    snackbar_link: WeakComponentLink<MatSnackbar>,
    snackbar_agent: Box<dyn Bridge<SnackbarAgent>>,
    snackbar_text: String,
}

pub enum SnackbarMessage {
    OpenWithText(String),
    Open,
    DefaultClosed(Option<String>),
}

impl Component for Snackbar {
    type Message = SnackbarMessage;
    type Properties = SnackbarProps;

    fn create(props: SnackbarProps, link: ComponentLink<Snackbar>) -> Self {
        let style = Style::create(
            "Snackbar",
            r#"
        "#,
        )
        .unwrap();
        let snackbar_agent =
            SnackbarAgent::bridge(link.callback(|text| SnackbarMessage::OpenWithText(text)));

        Self {
            style,
            props,
            link,
            snackbar_link: WeakComponentLink::default(),
            snackbar_agent,
            snackbar_text: String::from(""),
        }
    }
    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            SnackbarMessage::OpenWithText(text) => {
                self.snackbar_text = text;
                self.link.send_message(SnackbarMessage::Open);
                true
            }
            SnackbarMessage::Open => {
                self.snackbar_link.show();
                false
            }
            SnackbarMessage::DefaultClosed(str) => false,
            _ => false,
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        html! {
            <div class=self.style.to_string()>
                <section style="margin: 1em 0;">
                    <MatSnackbar
                        label_text={self.snackbar_text.to_string()}
                        snackbar_link=self.snackbar_link.clone()
                        onclosed=self.link.callback(SnackbarMessage::DefaultClosed)
                    >
                        <span slot="action">
                            <MatButton label="CLOSE" />
                        </span>
                    </MatSnackbar>
                </section>
            </div>
        }
    }
}
