use crate::utils::time::format_timestamp;
use css_in_rust::Style;
use yew::prelude::*;

pub enum PostCardMessage {}

#[derive(Properties, Clone)]
pub struct PostCardProps {
    pub title: String,
    pub summary: String,
    pub cover: String,
    pub create_at: u64,
}

pub struct PostCard {
    props: PostCardProps,
    link: ComponentLink<PostCard>,
    style: Style,
}

impl Component for PostCard {
    type Message = PostCardMessage;
    type Properties = PostCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "PostCard",
            r#"
            width: 280px;
            border-radius: 5px;
            overflow: hidden;
            margin: 0 30px;
            cursor: pointer;

            .post-card__cover {
                width: 100%;
                height: 166px;
                background-repeat: no-repeat;
                background-position: 50%;
                background-size: cover;
            }

            .post-card__body {
                padding: 12px;
                box-sizing: border-box;
                background: var(--base-color);
            }
            
            .post-card__body__title {
                color: var(--title-color);
                font-size: 17px;
                font-weight: bold;
                margin-top: 2px;
            }

            .post-card__body__summary {
                font-size: 13px;
                overflow: hidden;
                text-overflow: ellipsis;
                display: -webkit-box;
                -webkit-line-clamp: 3;
                -webkit-box-orient: vertical;
                margin-top: 6px;
            }

            .post-card__body__create-at {
                margin-top: 8px;
                color: var(--attach-text-color);
            }

            @media (max-width: 600px) {
                width: 100%;
                margin: 0;
                margin-bottom: 30px;
            }
        "#,
        )
        .unwrap();

        Self { props, link, style }
    }
    fn update(&mut self, _: <Self as yew::Component>::Message) -> bool {
        todo!()
    }
    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        let cover_styles = format!("background-image: url({})", self.props.cover);

        html! {
            <div class=format!("{} card", self.style.to_string())>
                <div class="post-card__cover" style=cover_styles></div>
                <div class="post-card__body">
                    <div class="post-card__body__title">
                        {&self.props.title}
                    </div>
                    <div class="post-card__body__summary">
                        {&self.props.summary}
                    </div>
                    <div class="post-card__body__create-at">
                        {format_timestamp(self.props.create_at, "%Y/%m/%d")}
                    </div>
                </div>
            </div>
        }
    }
}
