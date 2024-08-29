use std::{ffi::OsStr, path::Path};

#[derive(Clone)]
pub struct ThemeIcon {
    pub dark: String,
    pub light: String,
}

// A theme icon format looks like:
// filename.[light|dark].extension
// raw_icon_path is part of theme icon path which without theme suffix:
// filename.extension
// this function will parse raw_icon_path to theme_icon_path:
// {
//   dark: "filename.dark.extension",
//   light: "filename.light.extension
// }
pub fn parse_theme_icon(raw_icon_path: &str) -> ThemeIcon {
    let raw_path = Path::new(raw_icon_path);
    let file_path_without_extension = raw_path.with_extension("");
    let file_stem_with_path = file_path_without_extension.to_str().unwrap();
    let extension = raw_path.extension().and_then(OsStr::to_str).unwrap();
    let create_theme_icon =
        |theme: &'static str| format!("{file_stem_with_path}.{theme}.{extension}");

    ThemeIcon {
        light: create_theme_icon("light"),
        dark: create_theme_icon("dark"),
    }
}

// In some scenarios, we need to append a class to the existing class in components,
// this util function help concat the class prop to existing class
pub fn with_class_prop(raw_class: &str, class_prop: &Option<String>) -> String {
    match class_prop {
        Some(class_prop_value) => format!("{raw_class} {class_prop_value}"),
        None => raw_class.to_string(),
    }
}
