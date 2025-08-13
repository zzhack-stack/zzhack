// Markdown Processing
// Renders markdown to HTML with syntax highlighting and command execution

use crate::commands::{CommandExecutor, CommandResult, TerminalContext};
use crate::utils::{syntax_highlighter::SyntaxHighlighter, AppConfigService};
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

/// Fetch and render markdown file content to HTML with command execution support
pub async fn fetch_and_render_markdown_with_executor(
    file_path: &str,
    executor: &CommandExecutor,
) -> Result<String, String> {
    let content = super::fetcher::fetch_file_content(file_path).await?;
    Ok(render_markdown_to_html(&content, Some(executor)))
}

/// Render markdown content to HTML with syntax highlighting and optional command execution
pub fn render_markdown_to_html(markdown_input: &str, executor: Option<&CommandExecutor>) -> String {
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

    // Find the closing delimiter
    let lines: Vec<&str> = markdown_input.lines().collect();
    if lines.is_empty() {
        return markdown_input.to_string();
    }

    // Determine the closing delimiter pattern
    let delimiter = if lines[0] == "---" { "---" } else { "--" };

    // Find the end of the frontmatter
    for (i, line) in lines.iter().enumerate().skip(1) {
        if *line == delimiter {
            // Found closing delimiter, return content after it
            if i + 1 < lines.len() {
                return lines[i + 1..].join("\n");
            } else {
                return String::new(); // No content after frontmatter
            }
        }
    }

    // No closing delimiter found, return original content
    markdown_input.to_string()
}

/// Execute a "run" code block with the given executor
fn execute_run_block(code: &str, executor: &CommandExecutor) -> String {
    let mut output = String::new();
    let commands: Vec<&str> = code.lines().filter(|line| !line.trim().is_empty()).collect();

    for command_line in commands {
        let command_line = command_line.trim();
        if command_line.is_empty() {
            continue;
        }

        // Create a minimal context for command execution
        let context = TerminalContext {
            clear_screen: std::rc::Rc::new(|| {}),
            command_executor: executor,
            execute: std::rc::Rc::new(|_| {
                CommandResult::Error("Nested execute not supported in run blocks".to_string())
            }),
            app_config: AppConfigService::new(),
            set_theme: None,
        };

        let result = executor.execute(command_line, &context);
        
        output.push_str(&format!("<div class=\"command-line\">$ {}</div>", html_escape(command_line)));
        
        match result {
            CommandResult::Success(out) => {
                if !out.is_empty() {
                    output.push_str(&format!("<div class=\"command-output\">{}</div>", html_escape(&out)));
                }
            }
            CommandResult::Error(err) => {
                output.push_str(&format!("<div class=\"command-error\">Error: {}</div>", html_escape(&err)));
            }
            CommandResult::Html(html_content) => {
                output.push_str(&format!("<div class=\"command-html\">{}</div>", html_content));
            }
            CommandResult::Async(_) => {
                output.push_str("<div class=\"command-error\">Async commands not supported in run blocks</div>");
            }
        }
        output.push('\n');
    }

    format!("<div class=\"run-block\">{}</div>", output)
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}