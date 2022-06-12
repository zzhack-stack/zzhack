use crate::link::Link;
use router::RootRoutes;
use services::projects_service::projects_service::Project;
use stylist::style;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProjectCardProps {
    pub project: Project,
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let style = style!(
        r#"
        width: 384px;
        box-shadow: 0px 8px 50px 0px var(--card-shadow-color);
        border-radius: 8px;
        padding: 15px 20px;
        background-color: var(--base-color);
        box-sizing: border-box;
        margin-bottom: 30px;

        .name {
            font-size: 20px;
            color: var(text-color);
        }

        .desc {
            font-size: 13px;
            color: var(--sub-text-color);
        }

        .summary {
            word-break: break-all;
            -webkit-line-clamp: 9;
            -webkit-box-orient: vertical;
            overflow: hidden;
            text-overflow: ellipsis;
            display: -webkit-box;
            overflow: hidden;
            line-height: 25px;
            font-size: 14px;
        }

        .footer {
            text-align: right;
            font-size: 13px;
            margin-top: 10px;
        }

        .post-block {
            margin-top: 10px;
        }

        @media (max-width: 600px) {
            width: 100%;
        }
    "#
    )
    .unwrap();

    html! {
        <div class={style}>
            <Link out_href={props.project.addr.clone()}>
                <div class="name">
                    {&props.project.name}
                </div>
                <div class="desc">
                    {&props.project.desc}
                </div>
            </Link>
            {
                match &props.project.post {
                    Some(post) => {
                        html! {
                            <Link href={RootRoutes::Post {filename: post.filename.to_string() }}>
                                <div class="post-block">
                                    <div class="summary">
                                        {&post.desc}
                                    </div>
                                    <div class="footer">
                                        {"阅读全文"}
                                    </div>
                                </div>
                            </Link>
                        }
                    },
                    None => html! {},
                }
            }
        </div>
    }
}
