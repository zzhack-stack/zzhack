use crate::console_log;
use crate::services::article_service::User;
use crate::services::MarkdownService;
use crate::utils::theme::by_theme;
use css_in_rust::Style;
use material_yew::MatIconButton;
use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Properties, Clone)]
pub struct ArticleViewProps {
    pub content: String,
    pub user: User,
}

pub struct ArticleView {
    style: Style,
    props: ArticleViewProps,
    content: VNode,
}

impl Component for ArticleView {
    type Message = ();
    type Properties = ArticleViewProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "ArticleView",
            r#"
        "#,
        )
        .unwrap();
        let content = ArticleView::render_content(props.clone().content);

        Self {
            style,
            props,
            content,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.content = ArticleView::render_content(props.clone().content);
        self.props = props;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="container markdown-container">
                    {self.content.clone()}
                </div>
            </div>
        }
    }
}

impl ArticleView {
    fn render_content(content: String) -> VNode {
        let markdown_service = MarkdownService::new(content);
        let content =
            markdown_service.parse_to_element(by_theme("base16-ocean.light", "base16-ocean.light"));

        Html::VRef(content.into())
    }
}
