use services::theme_service::Theme;

const BASE_IMAGE_PATH: &'static str = "images";

pub fn with_assets(img_name: &str) -> String {
    format!("/{}/{}", BASE_IMAGE_PATH, img_name)
}

pub fn with_assets_by_theme(img_name: &str, theme: &Theme) -> String {
    format!("/{}/${}_{}", BASE_IMAGE_PATH, theme.into_str(), img_name)
}
