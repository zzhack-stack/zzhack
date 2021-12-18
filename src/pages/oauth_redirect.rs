use crate::services::github_service::{GitHubProfile, GitHubService};
use css_in_rust::Style;
use material_yew::MatCircularProgressFourColor;
use material_yew::MatLinearProgress;
use std::time::Duration;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::timeout::{TimeoutService, TimeoutTask};
use yew_router::agent::RouteRequest::ChangeRoute;
use yew_router::prelude::RouteAgent;

pub enum OAuthRedirectMessage {
    UpdateAccessKey(Option<String>),
    UpdateUserProfile(GitHubProfile),
    JumpOrigin,
    Nope,
}

#[derive(Properties, Clone)]
pub struct OAuthRedirectProps {
    pub code: String,
    pub redirect_url: String,
}

pub struct OAuthRedirect {
    pub props: OAuthRedirectProps,
    pub fetch_access_token_task: Option<FetchTask>,
    style: Style,
    link: ComponentLink<OAuthRedirect>,
    github_service: GitHubService,
    access_key: Option<String>,
    is_timeout: bool,
    profile: Option<GitHubProfile>,
    route_agent: Box<dyn Bridge<RouteAgent<()>>>,
    timeout_task: Option<TimeoutTask>,
}

impl Component for OAuthRedirect {
    type Message = OAuthRedirectMessage;
    type Properties = OAuthRedirectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let github_service = GitHubService::new();
        let style = Style::create(
            "OAuthRedirect",
            r#"
            
            .oauth-wrapper {
                position: relative;
                padding-bottom: 50px;
            }

            .oauth__card {
                box-shadow: rgba(100, 100, 111, 0.2) 0px 7px 29px 0px;
                background: var(--base-color);
                width: 600px;
                min-height: 200px;
                margin: 50px auto 0 auto;
                border-radius: 5px;
            }

            .boy-paint-illustration {
                position: absolute;
                left: -300px;
                width: 420px;
                top: 80px;
            }
            
            .boy-page-illustration {
                position: absolute;
                right: -300px;
                top: 250px;
            }

            .oauth__title {
                font-size: 28px;
                text-align: center;
                margin-top: 50px;
            }

            .oauth__nope {
                width: 100%;
                min-height: 200px;
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
            }

            .oauth__progress-bar {
                margin-top: 10px;
            }

            .oauth__progress-bar mwc-linear-progress {
                width: 200px;
            }

            .oauth__user-profile {
                margin-left: 10px;
            }

            .oauth__user-avatar {
                width: 80px;
                height: 80px;
                border-radius: 50%;
            }

            .oauth__user-info {
                display: flex;
                padding: 20px;
            }

            .oauth__user-name {
                font-size: 22px;
            }

            .oauth__user-login {
                color: var(--sub-text-color);
            }

            .oauth__success-footer {
                margin-top: 35px;
                text-align: center;
                display: flex;
                align-items: center;
                justify-content: center;
            }

            .oauth__success-footer mwc-circular-progress-four-color {
                width: 20px;
            }

            .oauth__success-footer__text {
                margin-right: 8px;
            }

            .oauth__user-wrapper {
                position: relative;
                overflow: hidden;
            }

            .oauth__github-icon {
                position: absolute;
                width: 150px;
                opacity: 0.6;
                transform: rotate(45deg);
                right: -10px;
                top: -10px;
            }

            @media (max-width: 600px) {
                .oauth-wrapper {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                }

                .oauth__github-icon {
                    width: 120px;
                }

                .oauth__card {
                    width: 100%;
                }

                .oauth__success-footer {
                    padding: 0 20px 20px 20px;
                }

                .boy-page-illustration {
                    position: static;
                    width: 65%;
                    margin-top: 30px;
                }

                .boy-paint-illustration {
                    position: static;
                    width: 65%;
                    margin-top: 30px;
                }
            }
        "#,
        )
        .unwrap();
        let route_agent = RouteAgent::bridge(link.callback(|_| OAuthRedirectMessage::Nope));

        Self {
            props,
            fetch_access_token_task: None,
            link,
            style,
            github_service,
            access_key: None,
            is_timeout: false,
            profile: None,
            route_agent,
            timeout_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            OAuthRedirectMessage::UpdateAccessKey(access_key) => {
                self.access_key = access_key;
                self.fetch_access_token_task = self.github_service.get_user_profile(
                    self.link
                        .callback(|profile| OAuthRedirectMessage::UpdateUserProfile(profile)),
                );

                true
            }
            OAuthRedirectMessage::JumpOrigin => {
                self.route_agent
                    .send(ChangeRoute(self.props.redirect_url.clone().into()));

                false
            }
            OAuthRedirectMessage::UpdateUserProfile(profile) => {
                self.profile = Some(profile);
                self.timeout_task = Some(TimeoutService::spawn(
                    Duration::from_millis(2000),
                    self.link.callback(|_| OAuthRedirectMessage::JumpOrigin),
                ));
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn rendered(&mut self, first_render: bool) {
        if !first_render {
            return;
        }

        self.fetch_access_token_task = Some(
            self.github_service.fetch_token(
                &self.props.code,
                self.link
                    .callback(|payload| OAuthRedirectMessage::UpdateAccessKey(payload)),
            ),
        );
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.style.to_string()>
                <div class="container oauth-wrapper">
                    <div class="oauth__title">
                        {"Verify ur account using Github OAuth"}
                    </div>
                    <img class="boy-paint-illustration" src="/images/boy_painting_illustration.svg" />
                    <img class="boy-page-illustration" src="/images/boy_page_illustration.svg" />
                    <div class="oauth__card">
                        {
                            match &self.profile {
                                None => html! {
                                    <div class="oauth__nope">
                                        {
                                            if self.is_timeout {
                                                html! {
                                                    <>{"Request access key timeout!"}</>
                                                }
                                            } else {
                                                html! {
                                                    <>
                                                        {"Requesting access key..."}
                                                        <div class="oauth__progress-bar">
                                                            <MatLinearProgress indeterminate=true />
                                                        </div>
                                                    </>
                                                }
                                            }
                                        }
                                    </div>
                                },
                                Some(profile) => html! {
                                    <div class="oauth__user-wrapper">
                                        <img class="oauth__github-icon" src="/images/github_dark.svg" />
                                        <div class="oauth__user-info">
                                            <img class="oauth__user-avatar" src=profile.avatar_url.clone() />
                                            <div class="oauth__user-profile">
                                                {
                                                    match &profile.name {
                                                        Some(name) => html! {<div class="oauth__user-name">{&name}</div>},
                                                        None => html! {}
                                                    }
                                                }
                                                <div class="oauth__user-login">{&profile.login}</div>
                                            </div>
                                        </div>
                                        <div class="oauth__success-footer">
                                            <div class="oauth__success-footer__text">
                                                {format!("Welcome to my blog {}! Jumping to the previous page...", &profile.login)}
                                            </div>
                                            <MatCircularProgressFourColor indeterminate=true />
                                        </div>
                                    </div>
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        }
    }
}
