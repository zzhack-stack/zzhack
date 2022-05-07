use services::theme_service::Theme;
use services::theme_service::ThemeService;
use std::rc::Rc;
use yew::prelude::*;

pub enum ThemeAction {
    UpdateTheme(Theme),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThemeState {
    pub theme: Theme,
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ThemeAction::UpdateTheme(theme) => {
                ThemeService::from_storage().set_theme(&theme);
                Rc::from(ThemeState { theme })
            }
        }
    }
}

pub type ThemeContext = UseReducerHandle<ThemeState>;

#[derive(Properties, Debug, PartialEq)]
pub struct ThemeProviderProps {
    pub children: Children,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme = use_reducer_eq(|| {
        let theme = ThemeService::from_storage().get_theme().clone();

        ThemeState { theme }
    });

    html! {
        <ContextProvider<ThemeContext> context={theme}>
            { props.children.clone() }
        </ContextProvider<ThemeContext>>
    }
}
