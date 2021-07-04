use crate::console_log;
use crate::services::api_service::Res;
use crate::services::article_service::article_service;
use crate::services::article_service::Article;
use crate::services::article_service::QueryRes;
use crate::services::ArticleService;
use crate::services::MarkdownService;
use css_in_rust::Style;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Response;

pub struct AboutMe {
    style: Style,
    task: Option<FetchTask>,
    content: String,
    num: i32,
    link: ComponentLink<Self>,
}

pub enum AboutMeMessage {
    UpdateContent(String),
}

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

        Self {
            style,
            task: None,
            content: "".to_string(),
            link,
            num: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.task = Some(article_service.get_article_by_label(
                "me",
                self.link.callback(|response: Res<QueryRes<Article>>| {
                    let Json(data) = response.into_body();
                    let article = data.unwrap().items[0].clone().body;

                    AboutMeMessage::UpdateContent(article)
                }),
            ));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AboutMeMessage::UpdateContent(content) => {
                console_log!("{}", content);
                self.content = content;

                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let markdown_service = MarkdownService::new(self.content.clone());

        html! {
            <div class=self.style.to_string()>
                <div class="container">
                    {Html::VRef(markdown_service.parse_to_element().into())}
                </div>
            </div>
        }
    }
}
