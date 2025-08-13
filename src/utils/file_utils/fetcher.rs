// File Content Fetcher
// Handles HTTP requests to fetch file content from the data directory

use super::super::config::build_data_url;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Fetch file content from the data directory
pub async fn fetch_file_content(file_path: &str) -> Result<String, String> {
    let url = build_data_url(file_path);

    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| format!("Failed to create request for {}", file_path))?;

    let window = web_sys::window().ok_or("No window object")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| format!("Network request failed for {}", file_path))?;

    let resp: Response = resp_value
        .dyn_into()
        .map_err(|_| "Failed to cast response".to_string())?;

    if !resp.ok() {
        return Err(format!(
            "Failed to fetch file {}: HTTP {}",
            file_path,
            resp.status()
        ));
    }

    let text_promise = resp.text().map_err(|_| "Failed to get text promise")?;
    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|_| "Failed to get text from response")?;

    let content = text_value
        .as_string()
        .ok_or("Failed to convert response to string")?;

    Ok(content)
}