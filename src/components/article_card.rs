use crate::services::article_service::ArticleWithMetadata;
use crate::AppRoutes;
use crate::Article;
use crate::RouteAgent;
use css_in_rust::Style;
use yew::prelude::*;
use yew_router::agent::RouteRequest::ChangeRoute;

#[derive(Properties, Clone)]
pub struct ArticleProps {
    pub article: ArticleWithMetadata,
}

pub struct ArticleCard {
    props: ArticleProps,
    style: Style,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    link: ComponentLink<ArticleCard>,
}

pub enum ArticleCardMessage {
    Nope,
    JumpDetail,
}

impl Component for ArticleCard {
    type Message = ArticleCardMessage;
    type Properties = ArticleProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "ArticleCard",
            r#"
            width: 100%;
            cursor: pointer;
            padding-bottom: 10px;

            .header {
                color: var(--shallow-gray);
            }

            .title {
                color: var(--title-color);
                font-size: 16px;
                font-weight: 700;
            }

            .desc {
                font-weight: 400;
                font-size: 13px;
                line-height: 22px;
                color: #86909c;
                display: -webkit-box;
                overflow: hidden;
                text-overflow: ellipsis;
                -webkit-box-orient: vertical;
                -webkit-line-clamp: 2;
            }

            .footer {
                display: flex;
                align-items: center;
                padding: 5px 0;
            }

            .filter {
                width: fit-content;
                background: #3970f5;
                color: white;
                padding: 2px 8px;
                font-size: 12px;
                border-radius: 1000px;
                margin-right: 7px;
            }

            .book-filter {
                background: #22a6b3;
            }

            .article-filter {
                background: #4834d4;
            }
        "#,
        )
        .unwrap();
        let route_agent = RouteAgent::bridge(link.callback(|_| ArticleCardMessage::Nope));

        Self {
            props,
            style,
            route_agent,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ArticleCardMessage::JumpDetail => {
                let article_with_metadata = self.props.article.clone();
                let article = article_with_metadata.article.clone();

                let route = if article_with_metadata.is_book {
                    AppRoutes::Books(article.number)
                } else {
                    AppRoutes::Articles(article.number)
                };

                self.route_agent.send(ChangeRoute(route.into()));

                true
            }
            ArticleCardMessage::Nope => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let article_with_metadata = self.props.article.clone();
        let article = article_with_metadata.article.clone();

        html! {
            <div onclick=self.link.callback(|_| ArticleCardMessage::JumpDetail) class=self.style.to_string()>
                <div class="header">
                    {article.user.login}
                </div>
                <div class="body">
                    <div class="title">{article.title}</div>
                    <div class="desc">{article.body}</div>
                </div>
                <div class="footer">
                    {
                        if article_with_metadata.is_book {html! {<div class="filter book-filter">{"书"}</div>}} else {html! {<div class="filter article-filter">{"文章"}</div>}}
                    }
                    {for article_with_metadata.filters.iter().map(|filter| html! {<div class="filter">{filter.name.clone()}</div>})}
                </div>
            </div>
        }
    }
}

// fn trim_body(body: String) -> String {
// body.
// }
