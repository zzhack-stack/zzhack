pub static HEADING_START: &'static str = "<div class='markdown-heading'>";
pub static HEADING_END: &'static str = "</div>";

pub static IMAGE_START: &'static str = "<img class='markdown-img'>";
pub static IMAGE_END: &'static str = "</img>";

pub fn render_code_block(code_block: String) -> String {
    format!(
        "<div class='markdown-code'>
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
