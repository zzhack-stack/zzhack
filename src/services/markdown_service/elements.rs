pub fn render_heading(content: String, level: u32) -> String {
    format!(
        "<a class='markdown-heading-anchor' href='#{}'><h{} class='markdown-heading'>{}</h{}></a>",
        content, level, content, level
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

pub fn render_code(code: String) -> String {
    format!("<span class='markdown-code'>{}</span>", code)
}
