use crate::components::Avatar;
use crate::console_log;
use crate::workers::markdown_worker::MarkdownInput;
use crate::workers::markdown_worker::MarkdownWorker;
use crate::Article;
use chrono::DateTime;
use css_in_rust::Style;
use material_yew::MatButton;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, Clone)]
pub struct ArticleViewProps {
    pub article: Article,
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
            
            .cover-header {
                width: 100%;
                height: 500px;
                display: flex;
                justify-content: center;
                align-items: center;
                overflow: hidden;
            }

            .cover-img {
                width: 100%;
                height: max-content;
            }

            .time-block {
                
            }

            .bottom-prompt {
                text-align: center;
                margin: 50px 0;
                font-size: 14px;
                color: var(--prompt-color);
            }

            .comments {

            }

            @media (max-width: 600px){                
                .cover-header {
                    height: auto;
                }

                .cover-img {
                    width: 100%;
                    height: auto;
                }
            }
        "#,
        )
        .unwrap();
        link.send_message(ArticleViewMessage::ParseContent(props.clone().article.body));
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
                    .set_href(self.props.article.user.html_url.as_str())
                    .unwrap();

                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // // true
        if self.props.article.title != props.article.title.clone() {
            self.props = props.clone();
            self.link
                .send_message(ArticleViewMessage::ParseContent(props.article.body.clone()));

            false
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let article = self.props.article.clone();

        html! {
            <div class=self.style.to_string()>

                {match article.cover {
                    Some(cover) => html! {
                        <div style=format!("background: {};", cover.background) class="cover-header">
                            <img class="cover-img" src={cover.cover} />
                        </div>
                    },
                    None => html! {}
                }}

                <div class="article-container">
                    <h1 class="title article-text">{article.title.clone()}</h1>
                    <div class="author">
                        <Avatar user={article.user.clone()} />
                        <div onclick=self.link.callback(|_| ArticleViewMessage::Follow)>
                            <MatButton  raised=true label="Follow!" />
                        </div>
                    </div>
                    // <div class="time-block">
                    //     <span>{article.created_at}</span>
                    //     <span>{article.updated_at}</span>
                    // </div>
                    <div class="markdown-container">
                        {match self.render_content.clone() {
                            Some(content) => content,
                            None => html! {"loading."}
                        }}
                    </div>
                </div>
                <div class="bottom-prompt">{"🎉 已经到底了，感谢你的阅读！"}</div>
            </div>
        }
    }
}

// fn format_date(time_str: String) -> (hour minutes) {
//     let rfc3339 = DateTime::parse_from_rfc3339(time_str.as_str()).unwrap();

// }
