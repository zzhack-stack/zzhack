use crate::routes::app_routes::AppRoutes;
use crate::services::provider_service::PostMetadata;
use crate::AppRouterAnchor;
use chrono::{DateTime, NaiveDateTime, Utc};
use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct PostCardProps {
    pub post_metadata: PostMetadata,
    pub category: &'static str,
}

pub struct PostCard {
    style: Style,
    props: PostCardProps,
}

impl PostCard {
    // fn format_create_at(timestamp: i64) -> String {
    //     let naive_date_time = NaiveDateTime::from_timestamp(timestamp, 0);
    //     let datetime:DateTime<Utc> = DateTime::from_utc(naive_date_time, Utc);
    //     let formated_data_time = datetime.format("%Y/%m/%d %H:%M");

    //     formated_data_time
    // }
}

impl Component for PostCard {
    type Message = ();
    type Properties = PostCardProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "PostCard",
            r#"

            .attach-info {
                font-size: 14px;
                font-weight: 300;
                color: var(--sub-text-color);
                line-height: 20px;
            }

            .title {
                font-size: 22px;
                font-weight: 500;
                color: var(--sub-text-color);
                line-height: 30px;
                margin0-top: 8px;
            }

            .summary {
                font-size: 14px;
                font-weight: 300;
                color: var(--sub-text-color);
                line-height: 20px;
                margin-top: 8px;
                overflow: hidden;
                text-overflow: ellipsis;
                display: -webkit-box;
                -webkit-box-orient: vertical;
                -webkit-line-clamp: 2;
            }

            .goto {
                text-decoration: none;
            }

            @media (max-width: 600px) {
            }
        "#,
        )
        .unwrap();

        Self { style, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let metadata = self.props.post_metadata.clone();

        html! {
                <div class=self.style.to_string()>
                    <AppRouterAnchor classes="goto" route={AppRoutes::Post(String::from(self.props.category), self.props.post_metadata.filename.clone())}>
                        <div class="wrapper">
                            <div class="attach-info">
                                {"Jul 22; Tags: help, life"}
                            </div>
                            <div class="title">{metadata.title.clone()}</div>
                            <div class="summary">{metadata.summary.clone()}</div>
                        </div>
                    </AppRouterAnchor>
                </div>
        }
    }
}
