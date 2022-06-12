use post::Post;
use ui::layout::BaseLayout;
use yew::prelude::*;
use yew_router::prelude::*;

use about::About;
use home::Home;
use links::Links;
use not_found::NotFound;
use projects::Projects;
use router::RootRoutes;

fn switch(routes: &RootRoutes) -> Html {
    match routes {
        RootRoutes::Home | RootRoutes::Root => html! { <Home /> },
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::About => html! { <About /> },
        RootRoutes::Post { filename } => html! {<Post filename={filename.clone()} />},
        RootRoutes::NotFound => html! { <NotFound />},
        RootRoutes::Technology => html! {
            <Redirect<RootRoutes> to={RootRoutes::Home}/>
        },
        RootRoutes::Links => html! {<Links />},
    }
}

#[function_component(RouteOutlet)]
pub fn route_outlet() -> Html {
    html! {
        <BrowserRouter>
            <BaseLayout>
                <Switch<RootRoutes> render={Switch::render(switch)} />
            </BaseLayout>
        </BrowserRouter>
    }
}
