use crate::services::article_service::User;
use css_in_rust::Style;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct AvatarProps {
    pub user: User,
}

pub struct Avatar {
    style: Style,
    props: AvatarProps,
}

impl Component for Avatar {
    type Message = ();
    type Properties = AvatarProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let style = Style::create(
            "Avatar",
            r#"
            display: flex;
            align-items: center;

            .avatar {
                width: 35px;
                border-radius: 50%;
                height: 35px;
                box-shadow: rgb(3 102 214 / 30%) 0px 0px 0px 3px;
            }

            .info {
                margin-left: 10px;
                font-weight: 700;
            }
        "#,
        )
        .unwrap();

        Self { style, props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let user = self.props.user.clone();

        html! {
            <a style="text-decoration: none;" href={user.html_url}>
                <div class=self.style.to_string()>
                    <img class="avatar" src={user.avatar_url} />
                    <div class="info">
                        <p class="name text">{"@"}{user.login}</p>
                    </div>
                </div>
            </a>
        }
    }
}
