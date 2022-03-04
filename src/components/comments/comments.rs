use crate::components::comments::CommentItem;
use crate::services::github_service::GitHubIssueComment;
use crate::services::github_service::{GitHubIssueComments, GitHubProfile};
use crate::services::snackbar_service::SnackbarService;
use crate::services::GitHubService;
use crate::CacheService;
use css_in_rust::Style;
use material_yew::MatButton;
use material_yew::MatCircularProgressFourColor;
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
    snackbar_service: SnackbarService,
    is_loading: bool,
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

            .comments-container {
                margin: auto;
            }

            .comments-wrapper {
                width: 100%;
                height: 200px;
                background: var(--base-color);
                border: 1px solid var(--border-color);
                border-radius: 5px;
                display: flex;
                justify-content: center;
                align-items: center;
                overflow: hidden;
                position: relative;
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

            .loading-block {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 200px;
                background: var(--base-color);
                opacity: 0.8;
                display: flex;
                justify-content: center;
                align-items: center;
            }

            @media (max-width: 600px) {
                .comment-list {
                    padding-top: 10px;            
                }
            }
        "#,
        )
        .unwrap();
        let github_service = GitHubService::new();
        let cache_service = CacheService::new();
        let access_token = cache_service.get_github_access_key();
        let github_profile = cache_service.get_github_profile();
        let snackbar_service = SnackbarService::new();

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
            snackbar_service,
            is_loading: false,
        }
    }
    fn update(&mut self, msg: <Self as yew::Component>::Message) -> bool {
        match msg {
            CommentsMessage::UpdateContent(content) => {
                self.comment_content = content;
                true
            }
            CommentsMessage::ReceivePublishCommentResponse(issue) => {
                self.comments.insert(0, issue);
                self.comment_content = "".to_string();
                self.is_loading = false;
                self.snackbar_service.send("Comment successful 🎉!");

                true
            }
            CommentsMessage::PublishComment => {
                self.is_loading = true;
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
                <div class="comments-container mini-container">
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
                                    <>
                                        <textarea value=self.comment_content.clone() onchange=self.link.callback(|content: ChangeData| CommentsMessage::UpdateContent(match content {
                                            Value(content) => content,
                                            _ => String::from("")
                                        })) class="comments-input" />
                                        {
                                            if self.is_loading {
                                                html! {
                                                    <div class="loading-block">
                                                        <MatCircularProgressFourColor indeterminate=true />
                                                    </div>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </>
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
                <div class="comment-list container">
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
