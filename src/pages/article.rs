use crate::components::Avatar;
use crate::console_log;
use crate::services::article_service::User;
use crate::services::MarkdownService;
use crate::utils::theme::by_theme;
use crate::workers::markdown_worker::MarkdownInput;
use crate::workers::markdown_worker::MarkdownWorker;
use css_in_rust::Style;
use material_yew::MatButton;
use material_yew::MatIconButton;
use std::borrow::Cow;
use std::time::Duration;
use yew::prelude::*;
use yew::services::{ConsoleService, Task, TimeoutService};
use yew::virtual_dom::VNode;

#[derive(Properties, Clone)]
pub struct ArticleViewProps {
    pub content: String,
    pub user: User,
    pub title: String,
}

pub enum ArticleViewMessage {
    ParseContent(String),
    UpdateContent(String),
    ParsedMarkdownContent(Option<VNode>),
    Follow,
}

pub struct ArticleView {
    style: Style,
    props: ArticleViewProps,
    render_content: Option<VNode>,
    link: ComponentLink<Self>,
    markdown_worker: Box<dyn Bridge<MarkdownWorker>>,
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

            .title {
                font-size: 35px;
            }

            .author {
                width: 100%;
                display: flex;
                align-items: center;
                justify-content: space-between;
                box-sizing: border-box;
                box-shadow: var(--undercover-color) 0px 8px 24px;
                background: var(--base-color);
                padding: 0 10px;
                border-radius: 5px;
            }
        "#,
        )
        .unwrap();
        link.send_message(ArticleViewMessage::ParseContent(props.clone().content));
        let markdown_worker = MarkdownWorker::bridge(
            link.callback(|_| ArticleViewMessage::ParsedMarkdownContent(None)),
        );

        Self {
            style,
            props,
            link,
            render_content: None,
            markdown_worker,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ArticleViewMessage::ParseContent(raw_content) => {
                self.link
                    .send_message(ArticleViewMessage::UpdateContent(raw_content.clone()));

                false
            }
            ArticleViewMessage::UpdateContent(raw_content) => {
                let callback = self.link.callback(|vnode: VNode| {
                    ArticleViewMessage::ParsedMarkdownContent(Some(vnode))
                });

                self.markdown_worker
                    .send(MarkdownInput::ParseContent(callback, raw_content));
                true
            }
            ArticleViewMessage::ParsedMarkdownContent(vnode) => match vnode {
                Some(vnode) => {
                    self.render_content = Some(vnode);

                    true
                }
                None => false,
            },
            ArticleViewMessage::Follow => {
                let window = web_sys::window().unwrap();
                window
                    .location()
                    .set_href(self.props.user.html_url.as_str());

                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // // true
        if self.props.content != props.content.clone() {
            self.props = props.clone();
            self.link
                .send_message(ArticleViewMessage::ParseContent(props.content.clone()));

            false
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="container">
                    <h1 class="title article-text">{self.props.title.clone()}</h1>
                    <div class="author">
                        <Avatar user={self.props.user.clone()} />
                        <div onclick=self.link.callback(|_| ArticleViewMessage::Follow)>
                            <MatButton  raised=true label="Follow!" />
                        </div>
                    </div>
                    <div class="markdown-container">
                        {match self.render_content.clone() {
                            Some(content) => content,
                            None => html! {"loading."}
                        }}
                    </div>
                </div>
            </div>
        }
    }
}

// fn render_content(content: String) -> VNode {
//     let markdown_service = MarkdownService::new(content);
//     let el =
//         markdown_service.parse_to_element(by_theme("base16-ocean.light", "base16-ocean.light"));

//     Html::VRef(el.into())
// }
