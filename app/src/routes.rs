use post::Post;
use ui::layout::BaseLayout;
use yew::prelude::*;
use yew_router::prelude::*;

use about::About;
use home::Home;
use not_found::NotFound;
use projects::Projects;
use router::RootRoutes;

fn switch(routes: &RootRoutes) -> Html {
    match routes {
        RootRoutes::Home | RootRoutes::Root => html! { <Home /> },
        RootRoutes::Projects => html! { <Projects /> },
        RootRoutes::About => html! { <About /> },
        RootRoutes::Post { title } => html! {<Post encoded_title={title.clone()} />},
        RootRoutes::NotFound => html! { <NotFound />},
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
