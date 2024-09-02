use cached::proc_macro::cached;
use serde_json::Value;

const EN: &'static str = include_str!("en.json");
const ZH: &'static str = include_str!("zh.json");

#[cached]
fn get_parsed_config() -> Value {
    let site_config = site_config::get_site_config();
    let language = site_config.root.language.as_str();
    let language_source = match language {
        "zh" => ZH,
        "en" => EN,
        _ => panic!("Unsupported language"),
    };

    serde_json::from_str(language_source).expect("Wrong format of language json")
}

pub fn t(key: &str) -> String {
    let config = get_parsed_config();
    let value = config.get(key).expect("Key not found");

    value.as_str().expect("Value is not a string").to_string()
}
