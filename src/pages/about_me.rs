use crate::services::article_service::article_service;
use crate::services::MarkdownService;
use crate::utils::theme::by_theme;
use css_in_rust::Style;
use web_sys::Element;
use yew::prelude::*;

pub struct AboutMe {
    style: Style,
    content: Element,
}

pub enum AboutMeMessage {}

impl Component for AboutMe {
    type Message = AboutMeMessage;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "AboutMe",
            r#"
            min-height: calc(100% - 48px);
        "#,
        )
        .unwrap();
        let markdown_service = unsafe {
            MarkdownService::new(article_service.get_article_by_label("me").unwrap().body)
        };
        let content =
            markdown_service.parse_to_element(by_theme("base16-ocean.light", "base16-ocean.light"));

        Self { style, content }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="container markdown-container">
                    {Html::VRef(self.content.clone().into())}
                </div>
            </div>
        }
    }
}
