// File operation utilities
// Provides functions for fetching and processing file content

use super::syntax_highlighter::SyntaxHighlighter;
use super::{config::build_data_url, AppConfigService};
use crate::commands::{CommandExecutor, CommandResult};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};
use std::rc::Rc;
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
    Ok(render_markdown_to_html(&content, None))
}

/// Fetch and render markdown file content to HTML with command execution support
pub async fn fetch_and_render_markdown_with_executor(
    file_path: &str,
    executor: &CommandExecutor,
) -> Result<String, String> {
    let content = fetch_file_content(file_path).await?;
    Ok(render_markdown_to_html(&content, Some(executor)))
}

/// Render markdown content to HTML with syntax highlighting and optional command execution
fn render_markdown_to_html(markdown_input: &str, executor: Option<&CommandExecutor>) -> String {
    // Remove frontmatter metadata before parsing
    let content_without_metadata = strip_frontmatter(markdown_input);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&content_without_metadata, options);
    let highlighter = SyntaxHighlighter::new();

    // Process events to add syntax highlighting
    let events: Vec<Event> = parser.collect();
    let mut processed_events = Vec::new();

    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                in_code_block = true;
                code_block_lang = lang.to_string();
                code_block_content.clear();
                // Don't push the original start tag, we'll create our own
            }
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                if in_code_block {
                    let highlighted_html = if code_block_lang == "run" {
                        // Handle executable code block
                        if let Some(exec) = executor {
                            execute_run_block(&code_block_content, exec)
                        } else {
                            // Fallback to regular code block for run blocks without executor
                            format!(
                                "<pre><code>{}</code></pre>",
                                html_escape(&code_block_content)
                            )
                        }
                    } else if code_block_lang.is_empty() {
                        // No language specified, just escape HTML - use regular markdown rendering
                        format!(
                            "<pre><code>{}</code></pre>",
                            html_escape(&code_block_content)
                        )
                    } else {
                        // Apply syntax highlighting - use regular markdown rendering
                        highlighter.highlight_code(&code_block_content, &code_block_lang)
                    };

                    // Push as raw HTML
                    processed_events.push(Event::Html(highlighted_html.into()));

                    in_code_block = false;
                    code_block_lang.clear();
                    code_block_content.clear();
                }
                // Don't push the original end tag
            }
            Event::Text(text) if in_code_block => {
                // Collect code block content
                code_block_content.push_str(&text);
                // Don't push the text event, we'll handle it when the block ends
            }
            _ => {
                // For all other events, pass them through unchanged
                processed_events.push(event);
            }
        }
    }

    // Convert processed events to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, processed_events.into_iter());

    // Wrap the rendered HTML in a div with markdown-content class
    format!("<div class=\"markdown-content\">{}</div>", html_output)
}

/// Strip frontmatter metadata from markdown content
fn strip_frontmatter(markdown_input: &str) -> String {
    // Check if content starts with frontmatter delimiter
    if !markdown_input.starts_with("--\n") && !markdown_input.starts_with("---\n") {
        return markdown_input.to_string();
    }

    let delimiter = if markdown_input.starts_with("---\n") {
        "---"
    } else {
        "--"
    };
    let lines: Vec<&str> = markdown_input.split('\n').collect();

    // Find the end of frontmatter
    let mut end_index = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == delimiter {
            end_index = Some(i);
            break;
        }
    }

    if let Some(end_idx) = end_index {
        // Return content after the closing delimiter
        lines[(end_idx + 1)..].join("\n")
    } else {
        // If no closing delimiter found, return original content
        markdown_input.to_string()
    }
}

/// Execute a run code block with multiple commands
fn execute_run_block(code_content: &str, executor: &CommandExecutor) -> String {
    let commands: Vec<&str> = code_content
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    if commands.is_empty() {
        return String::new();
    }

    let mut last_result: Option<String> = None;

    // Execute each command sequentially
    for command in commands {
        let parts = parse_command_with_quotes(command);

        if parts.is_empty() {
            continue;
        }

        // Use a minimal context for command execution
        let dummy_clear = Rc::new(|| {});
        let dummy_output = Rc::new(|_: String| {});
        let dummy_execute = Rc::new(|_: &str| CommandResult::Success(String::new()));

        let context = crate::commands::TerminalContext {
            clear_screen: dummy_clear,
            output_html: dummy_output,
            command_executor: executor,
            execute: dummy_execute,
            app_config: AppConfigService::new(),
        };

        match executor.execute_command(&parts[0], &parts[1..], &context) {
            CommandResult::Success(output) => {
                if !output.trim().is_empty() {
                    last_result = Some(output);
                }
            }
            CommandResult::Html(html_output) => {
                if !html_output.trim().is_empty() {
                    last_result = Some(html_output);
                }
            }
            CommandResult::Error(_) => {
                // Continue executing other commands even if one fails
                continue;
            }
            CommandResult::Async(_) => {
                // Skip async commands in run blocks for now
                continue;
            }
        }
    }

    // Return the output of the last command that produced output, or empty if none
    match last_result {
        Some(output) => {
            // Return output as-is, whether it's HTML or plain text
            output
        }
        None => String::new(),
    }
}

/// Parse command line with quote handling
fn parse_command_with_quotes(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current_part = String::new();
    let mut in_quotes = false;
    let mut escape_next = false;

    for ch in input.chars() {
        if escape_next {
            current_part.push(ch);
            escape_next = false;
        } else if ch == '\\' {
            escape_next = true;
        } else if ch == '"' {
            in_quotes = !in_quotes;
        } else if ch.is_whitespace() && !in_quotes {
            if !current_part.is_empty() {
                parts.push(current_part.clone());
                current_part.clear();
            }
        } else {
            current_part.push(ch);
        }
    }

    if !current_part.is_empty() {
        parts.push(current_part);
    }

    parts
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
