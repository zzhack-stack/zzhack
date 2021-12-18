use crate::components::comments::CommentItem;
use crate::console_log;
use crate::services::github_service::GitHubIssueComment;
use crate::services::github_service::{GitHubIssueComments, GitHubProfile};
use crate::services::GitHubService;
use crate::CacheService;
use css_in_rust::Style;
use material_yew::MatButton;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::ChangeData::Value;

#[derive(Properties, Clone)]
pub struct CommentsProps {
    pub issue_number: i32,
}

pub struct Comments {
    style: Style,
    pub props: CommentsProps,
    pub link: ComponentLink<Comments>,
    github_service: GitHubService,
    access_token: Option<String>,
    github_profile: Option<GitHubProfile>,
    comment_content: String,
    create_issue_task: Option<FetchTask>,
    fetch_comments_task: Option<FetchTask>,
    comments: GitHubIssueComments,
}

pub enum CommentsMessage {
    PublishComment,
    UpdateContent(String),
    ReceivePublishCommentResponse(GitHubIssueComment),
    UpdateComments(GitHubIssueComments),
}

impl Component for Comments {
    type Message = CommentsMessage;
    type Properties = CommentsProps;
    fn create(props: CommentsProps, link: ComponentLink<Comments>) -> Self {
        let style = Style::create(
            "Comments",
            r#"
            margin-top: 60px;

            .comments-wrapper {
                width: 100%;
                height: 200px;
                background: var(--undercover-color);
                border: 1px solid var(--border-color);
                border-radius: 5px;
                display: flex;
                justify-content: center;
                align-items: center;
                overflow: hidden;
            }

            .sign-with-github {
                display: flex;
                background: #000;
                width: 200px;
                cursor: pointer;
                padding: 10px;
                border-radius: 5px;
                align-items: center;
                justify-content: center;
                transition: all 0.3s;
                color: white;
            }

            .sign-with-github:hover {
                opacity: 0.7;
            }

            .comments__github-icon {
                width: 32px;
                margin-left: 10px;
            }

            .comments-title {
                font-size: 30px;
            }

            .comments-desc {
                color: var(--sub-text-color);
                font-size: 13px;
            }

            .comments-input {
                background: var(--undercover-color);
                border: none;
                outline: none;
                width: 100%;
                height: 200px;
                padding: 10px;
                box-sizing: border-box;
                resize: none;
            }

            .action-bar {
                margin-top: 10px;
                display: flex;
                justify-content: flex-end;
            }

            .comment-list {
                width: 100%;
                min-height: 50px;
                margin-top: 50px;
            }
        "#,
        )
        .unwrap();
        let github_service = GitHubService::new();
        let cache_service = CacheService::new();
        let access_token = cache_service.get_github_access_key();
        let github_profile = cache_service.get_github_profile();

        Self {
            style,
            props,
            link,
            github_service,
            github_profile,
            access_token,
            comment_content: String::from(""),
            create_issue_task: None,
            fetch_comments_task: None,
            comments: vec![],
        }
    }
    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            CommentsMessage::UpdateContent(content) => {
                self.comment_content = content;
                false
            }
            CommentsMessage::ReceivePublishCommentResponse(issue) => {
                self.comments.insert(0, issue);
                true
            }
            CommentsMessage::PublishComment => {
                self.create_issue_task =
                    Some(self.github_service.create_issue(
                        self.props.issue_number,
                        &self.comment_content,
                        self.link.callback(|issue| {
                            CommentsMessage::ReceivePublishCommentResponse(issue)
                        }),
                    ));
                true
            }
            CommentsMessage::UpdateComments(comments) => {
                self.comments = comments;
                true
            }
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn rendered(&mut self, is_rendered: bool) {
        if is_rendered {
            self.fetch_comments_task = Some(
                self.github_service.get_issue_comments(
                    self.props.issue_number,
                    self.link
                        .callback(|comments| CommentsMessage::UpdateComments(comments)),
                ),
            );
        }
    }

    fn view(&self) -> yew::virtual_dom::VNode {
        html! {
            <div class=self.style.to_string()>
                <div class="container">
                    <div class="comments-header">
                        <div class="comments-title">{"Comments"}</div>
                        <div class="comments-desc">{"The data source of comments comes from GitHub issues."}</div>
                    </div>
                    <div class="comments-wrapper">
                        {
                            if self.access_token.is_none() {
                                html! {
                                    <a class="non-style-link" href=GitHubService::get_oauth_url()>
                                        <div class="sign-with-github">
                                            {"Sign in with GitHub"}
                                            <img class="comments__github-icon" src="/images/github_light.svg" />
                                        </div>
                                    </a>
                                }
                            } else {
                                html! {
                                    <textarea onchange=self.link.callback(|content: ChangeData| CommentsMessage::UpdateContent(match content {
                                        Value(content) => content,
                                        _ => String::from("")
                                    })) class="comments-input" />
                                }
                            }
                        }
                    </div>
                    {
                        if self.access_token.is_some() {
                            html! {
                                <div class="action-bar">
                                    <div onclick=self.link.callback(|_| CommentsMessage::PublishComment)>
                                        <MatButton label="Send" raised=true/>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div class="comment-list">
                    {
                        for self.comments.iter().map(|comment| {
                            html! {
                                <CommentItem comment=comment.clone() />
                            }
                        })
                    }
                </div>
            </div>
        }
    }
}
