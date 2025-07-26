use crate::utils::AppConfigService;
use yew::prelude::*;

/// Hook for using AppConfig in Yew components
#[hook]
pub fn use_app_config() -> UseStateHandle<AppConfigService> {
    use_state(|| AppConfigService::new())
}