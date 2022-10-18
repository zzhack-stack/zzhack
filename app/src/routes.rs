use ui::layout::BaseLayout;
use yew::prelude::*;
use yew_router::prelude::*;

use router::RootRoutes;

use crate::routes_switch::switch;

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
