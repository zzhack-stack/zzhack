use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct OAuthRedirectProps {
    pub code: String,
    pub redirect_url: String,
}

pub struct OAuthRedirect {
    pub props: OAuthRedirectProps,
}

impl Component for OAuthRedirect {
    type Message = ();
    type Properties = OAuthRedirectProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
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
