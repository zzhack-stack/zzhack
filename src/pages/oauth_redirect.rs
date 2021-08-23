use crate::console_log;
use crate::services::api_service::api_service;
use crate::QueryRes;
use crate::Res;
use serde::Deserialize;
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::FetchService;

pub enum OAuthRedirectMessage {
    FetchGitHubAK(String),
}

#[derive(Properties, Clone)]
pub struct OAuthRedirectProps {
    pub code: String,
    pub redirect_url: String,
}

pub struct OAuthRedirect {
    pub props: OAuthRedirectProps,
    pub task: FetchTask,
}

#[derive(Deserialize)]
pub struct GitHubAccessKey {
    pub access_key: String,
}

impl Component for OAuthRedirect {
    type Message = OAuthRedirectMessage;
    type Properties = OAuthRedirectProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = fetch_github_ak(
            &props.code,
            link.callback(|response: Res<GitHubAccessKey>| {
                let Json(data) = response.into_body();

                OAuthRedirectMessage::FetchGitHubAK(data.unwrap().access_key)
            }),
        );

        Self { props, task }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            OAuthRedirectMessage::FetchGitHubAK(ak) => {
                console_log!("{}", ak);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {"Oauth redirect"}
            </div>
        }
    }
}

fn fetch_github_ak(code: &str, callback: Callback<Res<GitHubAccessKey>>) -> FetchTask {
    let request = api_service.get(String::from(format!("/github/ak?code={}", code)));

    FetchService::fetch(request, callback).unwrap()
}
