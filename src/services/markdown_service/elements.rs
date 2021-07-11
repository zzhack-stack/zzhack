use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubRenderBlock {
    pub url: String,
    pub repo: String,
    pub description: String,
}

pub fn render_heading(content: String, level: u32) -> String {
    format!(
        "<a class='markdown-heading-anchor' href='#{}'><h{} class='markdown-heading' id={}>{}</h{}></a>",
        content, level, content, content, level
    )
}

pub fn render_code_block(code_block: String) -> String {
    format!(
        "<div class='markdown-code-block'>
                <div class='markdown-mac-control-bars'>
                    <div class='markdown-mac-close-bar'></div>
                    <div class='markdown-mac-min-bar'></div>
                    <div class='markdown-mac-max-bar'></div>
                </div>
                {}
            </div>",
        code_block
    )
}

pub fn render_text(text: String) -> String {
    format!("<span class='markdown-text'>{}</span>", text)
}

pub fn render_image(url: String, alt: String) -> String {
    format!(
        "<div class='markdown-img-container'>
            <img class='markdown-img' src='{}' alt='{}' />
            <a href='{}' class='markdown-img-alt'>{}</a>
        </div>",
        url, alt, url, alt
    )
}

pub fn render_github_render_block(github_render_block: GitHubRenderBlock) -> String {
    format!(
        "<div class='markdown-github-render-block-container'>
        <a class='markdown-github-render-block-link' href={}>
            <div class='markdown-github-render-block'>
                <img class='markdown-github-render-block-icon' src='/images/github_light.svg' />
                <div class='markdown-github-render-block-info'>
                    <div class='markdown-github-render-block-repo'>{}</div>
                    <div class='markdown-github-render-block-desc'>{}</div>
                </div>
                <div class='markdown-github-render-block-goto'>
                    <img class='markdown-github-render-block-goto-icon' src='/images/forward.svg' />
                </div>
            </div>  
        </a>  
    </div>",
        github_render_block.url, github_render_block.repo, github_render_block.description
    )
}

pub fn render_code(code: String) -> String {
    format!("<span class='markdown-code'>{}</span>", code)
}
