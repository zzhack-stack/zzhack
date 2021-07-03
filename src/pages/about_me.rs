use crate::console_log;
use crate::services::ArticleService;
use css_in_rust::Style;
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
            self.task = Some(ArticleService::get(
                "/about_me/about_me.md",
                self.link
                    .callback(|response: Response<Result<String, anyhow::Error>>| {
                        let data = response.into_body().unwrap();
                        AboutMeMessage::UpdateContent(data)
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
        html! {
            <div class=self.style.to_string()>
                <div class="container">
                    {self.content.as_str()}
                </div>
            </div>
        }
    }
}
