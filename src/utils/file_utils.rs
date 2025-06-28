// File operation utilities
// Provides functions for fetching and processing file content

use super::config::build_data_url;
use pulldown_cmark::{html, Options, Parser};
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

/// Fetch and render markdown file content to HTML
pub async fn fetch_and_render_markdown(file_path: &str) -> Result<String, String> {
    let content = fetch_file_content(file_path).await?;
    Ok(render_markdown_to_html(&content))
}

/// Render markdown content to HTML with CSS classes
fn render_markdown_to_html(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Wrap the rendered HTML in a div with markdown-content class
    format!("<div class=\"markdown-content\">{}</div>", html_output)
}
