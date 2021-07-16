use crate::console_log;
use crate::services::article_service::User;
use crate::services::MarkdownService;
use crate::utils::theme::by_theme;
use css_in_rust::Style;
use material_yew::MatIconButton;
use std::time::Duration;
use yew::prelude::*;
use yew::services::{ConsoleService, Task, TimeoutService};
use yew::virtual_dom::VNode;

#[derive(Properties, Clone)]
pub struct ArticleViewProps {
    pub content: String,
    pub user: User,
}

pub enum ArticleViewMessage {
    ParseContent(String),
    UpdateContent(String),
}

pub struct ArticleView {
    style: Style,
    props: ArticleViewProps,
    render_content: Option<VNode>,
    link: ComponentLink<Self>,
}

impl Component for ArticleView {
    type Message = ArticleViewMessage;
    type Properties = ArticleViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "ArticleView",
            r#"
            width: 100%;
            min-height: 100vh;
            padding-bottom: 100px;
        "#,
        )
        .unwrap();
        link.send_message(ArticleViewMessage::ParseContent(props.clone().content));

        Self {
            style,
            props,
            link,
            render_content: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ArticleViewMessage::ParseContent(raw_content) => {
                self.link
                    .send_message(ArticleViewMessage::UpdateContent(raw_content.clone()));

                true
            }
            ArticleViewMessage::UpdateContent(raw_content) => {
                self.render_content = Some(render_content(raw_content));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // // true
        if self.props.content != props.content.clone() {
            self.props = props.clone();
            self.link
                .send_message(ArticleViewMessage::ParseContent(props.content.clone()));

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="container markdown-container">
                    {match self.render_content.clone() {
                        Some(content) => content,
                        None => html! {"loading."}
                    }}
                </div>
            </div>
        }
    }
}

fn render_content(content: String) -> VNode {
    let markdown_service = MarkdownService::new(content);
    let el =
        markdown_service.parse_to_element(by_theme("base16-ocean.light", "base16-ocean.light"));

    Html::VRef(el.into())
}
