use crate::services::theme_service::ThemeService;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn into_str(self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
        }
    }

    pub fn from(theme: &str) -> Theme {
        match theme {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            _ => Theme::Light,
        }
    }

    pub fn eq(&self, other: &Theme) -> bool {
        self.clone().into_str() == other.clone().into_str()
    }

    pub fn nq(&self, other: &Theme) -> bool {
        !self.eq(other)
    }
}

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
