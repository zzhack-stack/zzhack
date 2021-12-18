use crate::services::github_service::GitHubIssueComment;
use crate::utils::time::format_time_string;
use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct CommentsProps {
    pub comment: GitHubIssueComment,
}

pub struct CommentItem {
    style: Style,
    props: CommentsProps,
    link: ComponentLink<CommentItem>,
}

pub enum CommentItemMessage {}

impl Component for CommentItem {
    type Message = CommentItemMessage;
    type Properties = CommentsProps;

    fn create(props: CommentsProps, link: ComponentLink<CommentItem>) -> Self {
        let style = Style::create(
            "CommentItem",
            r#"
            padding: 17px;
            margin-bottom: 30px;
            border-radius: 5px;
            box-sizing: border-box;
            box-shadow: rgb(0 0 0 / 5%) 0px 6px 24px 0px, rgb(0 0 0 / 8%) 0px 0px 0px 1px;
            background: var(--comment-item-color);

            .comment-item__header {
                display: flex;
                align-items: center;
                justify-content: space-between;
            }

            .comment-item__body {
                word-break: break-all;
                padding: 17px 0;
            }

            .comment-item__updated_at {
                font-size: 14px;
                color: var(--attach-text-color);
            }

            .comment-item__created-at {
                font-size: 14px;
                color: var(--attach-text-color);
            }

            .comment-item__footer {

            }

            .comment-item__login {
                font-weight: bold;
                margin: 0 8px;
                font-size: 17px;
            }

            .comment-item__avatar {
                width: 40px;
                height: 40px;
                border-radius: 50%;
            }

            .comment-item__header__user {
                display: flex;
                align-items: center;
            }
        "#,
        )
        .unwrap();
        Self { style, props, link }
    }

    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.comment = props.comment;
        true
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        let comment = &self.props.comment;

        html! {
            <div class=format!("{} container", self.style.to_string())>
                <div class="comment-item__header">
                    <div class="comment-item__header__user">
                    <img class="comment-item__avatar" src=comment.user.avatar_url.clone() />
                    <span class="comment-item__login">{&comment.user.login}</span>
                    </div>
                    <span class="comment-item__created-at">
                        {format!("Latest updated at {}", format_time(&comment.created_at))}
                    </span>
                </div>
                <div class="comment-item__body">
                    {&comment.body}
                </div>
            </div>
        }
    }
}

fn format_time(timestring: &str) -> String {
    format_time_string(timestring, "%Y/%m/%d %H:%M")
}
