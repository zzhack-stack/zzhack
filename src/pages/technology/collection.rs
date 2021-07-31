use crate::article_service;
use crate::components::ArticleCard;
use crate::services::article_service::ArticleWithMetadata;
use crate::Article;
use crate::Footer;
use css_in_rust::Style;
use yew::prelude::*;

pub struct Collection {
    articles: Vec<ArticleWithMetadata>,
    style: Style,
}

impl Component for Collection {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let articles =
            unsafe { article_service.get_articles_by_labels_with_metadata(vec!["tab:collection"]) };
        let style = Style::create(
            "Collection",
            r#"
            .article-card {
                border-top: 1px solid rgba(0, 0, 0, 0.12);
                padding-top: 10px;
            }
        "#,
        )
        .unwrap();

        Self { articles, style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div style="display: flex;" class="container">
                    <div style="margin: 35px 0;" class="articles">
                        {for self.articles.iter().map(|article| {
                            let idx = self.articles.iter().position(|item| item.article.number == article.article.number).unwrap();

                            html! {
                                <div class=if idx != 0 {"article-card"} else {""}>
                                    <ArticleCard article={article.clone()} />
                                </div>
                            }
                        })}
                    </div>
                </div>
            </div>
        }
    }
}
