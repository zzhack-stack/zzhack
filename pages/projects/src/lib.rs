use services::projects_service::projects_service::Project;
use services::projects_service::projects_service::PROJECTS_SERVICE;
use stylist::style;
use ui::contact::ContactType;
use ui::image::ThemeImage;
use ui::link::Link;
use ui::ProjectCard;
use utils::theme::is_on_mobile;
use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let style = style!(
        r"
        margin-top: 30px;

        & > a > img {
            width: 100%;
        }

        .alert{
            width: 100%;
            background-color: var(--alert-color);
            padding: 14px 19px;
            box-sizing: border-box;
            border-radius: 10px;
            margin: 15px 0;
        }

        .alert > p {
            color: var(--text-color);
            line-height: 20px;
            font-size: 13px;
        }

        .alert > p > a {
            line-height: 20px;
            font-size: 13px;
            color: var(--blue);
        }

        .cards {
            display: flex;
            justify-content: space-between;
            margin-top: 40px;
            margin-bottom: 45px;
        }

        @media (max-width: 600px) {
            .cards {
                flex-direction: column;
            }
        }
    "
    )
    .unwrap();
    let render_project_card = |projects: Vec<Project>| -> Html {
        projects
            .into_iter()
            .map(|project| {
                html! {
                    <ProjectCard project={project} />
                }
            })
            .collect::<Html>()
    };
    let render_waterfall_flow = || -> Html {
        let (odd, even) = PROJECTS_SERVICE.get_projects_by_odd_even();

        html! {
            <>
                <div class="odd-cards">
                {render_project_card(odd)}
                </div>
                <div class="even-cards">
                    {render_project_card(even)}
                </div>
            </>
        }
    };
    let render_linear_flow = || -> Html {
        let projects = PROJECTS_SERVICE.get_projects();

        render_project_card(projects)
    };

    let render_target_vnode = if is_on_mobile() {
        render_linear_flow()
    } else {
        render_waterfall_flow()
    };

    html! {
        <div class={style}>
            <Link out_href={ContactType::GitHub.into_lnk()}>
                <ThemeImage  source="projects_banner.svg" is_reactive={true} />
            </Link>
            <div class="alert">
                <p>{"Here are a few cool projects I've built, and been involved with!"}</p>
                <p>
                    {"If you wanna collaborate on something, feel free to send me an email "}
                    <Link out_href={ContactType::Email.into_lnk()}>{"here"}</Link>
                    {" ✌️"}
                </p>
            </div>
            <div class="cards">
                {render_target_vnode}
            </div>
        </div>
    }
}
