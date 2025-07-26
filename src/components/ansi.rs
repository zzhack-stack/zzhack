// ANSI Escape Sequence Processing
// This module handles parsing and rendering of ANSI escape sequences for terminal colors

use yew::prelude::*;

/// Represents a styled text segment with ANSI color information
#[derive(Clone, PartialEq)]
pub struct AnsiSegment {
    pub text: String,
    pub color: Option<String>,
    pub bold: bool,
    pub italic: bool,
}

/// Parse ANSI escape sequences and convert them to HTML spans
pub fn parse_ansi_text(text: &str) -> Vec<AnsiSegment> {
    let mut segments = Vec::new();
    let mut current_segment = AnsiSegment {
        text: String::new(),
        color: None,
        bold: false,
        italic: false,
    };

    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            // Found ANSI escape sequence
            chars.next(); // consume '['

            // Save current segment if it has text
            if !current_segment.text.is_empty() {
                segments.push(current_segment.clone());
                current_segment.text.clear();
            }

            // Parse the escape sequence
            let mut code = String::new();
            while let Some(ch) = chars.next() {
                if ch.is_ascii_alphabetic() {
                    break;
                }
                code.push(ch);
            }

            // Apply the ANSI code
            apply_ansi_code(&code, &mut current_segment);
        } else {
            current_segment.text.push(ch);
        }
    }

    // Add the final segment
    if !current_segment.text.is_empty() {
        segments.push(current_segment);
    }

    segments
}

/// Apply ANSI color codes to a text segment
pub fn apply_ansi_code(code: &str, segment: &mut AnsiSegment) {
    let codes: Vec<u32> = code.split(';').filter_map(|s| s.parse().ok()).collect();

    for &code in &codes {
        match code {
            0 => {
                // Reset all formatting
                segment.color = None;
                segment.bold = false;
                segment.italic = false;
            }
            1 => segment.bold = true,
            3 => segment.italic = true,
            22 => segment.bold = false,
            23 => segment.italic = false,
            30 => segment.color = Some("ansi-black".to_string()),        // Black
            31 => segment.color = Some("ansi-red".to_string()),          // Red
            32 => segment.color = Some("ansi-green".to_string()),        // Green
            33 => segment.color = Some("ansi-yellow".to_string()),       // Yellow
            34 => segment.color = Some("ansi-blue".to_string()),         // Blue
            35 => segment.color = Some("ansi-magenta".to_string()),      // Magenta
            36 => segment.color = Some("ansi-cyan".to_string()),         // Cyan
            37 => segment.color = Some("ansi-white".to_string()),        // White
            90 => segment.color = Some("ansi-bright-black".to_string()), // Bright Black (Gray)
            91 => segment.color = Some("ansi-bright-red".to_string()),   // Bright Red
            92 => segment.color = Some("ansi-bright-green".to_string()), // Bright Green
            93 => segment.color = Some("ansi-bright-yellow".to_string()), // Bright Yellow
            94 => segment.color = Some("ansi-bright-blue".to_string()),  // Bright Blue
            95 => segment.color = Some("ansi-bright-magenta".to_string()), // Bright Magenta
            96 => segment.color = Some("ansi-bright-cyan".to_string()),  // Bright Cyan
            97 => segment.color = Some("ansi-bright-white".to_string()), // Bright White
            _ => {}                                            // Ignore unknown codes
        }
    }
}

/// Render ANSI segments as HTML with proper styling
pub fn render_ansi_segments(segments: &[AnsiSegment]) -> Html {
    html! {
        <>
            {for segments.iter().map(|segment| {
                let mut classes = vec!["text-sm", "font-mono"];

                // Apply color class
                if let Some(color_class) = &segment.color {
                    classes.push(color_class);
                } else {
                    // Default to terminal text color if no color specified
                    classes.push("text-terminal-text");
                }

                // Apply font weight
                if segment.bold {
                    classes.push("font-bold");
                }

                // Apply font style
                if segment.italic {
                    classes.push("italic");
                }

                html! {
                    <span class={classes.join(" ")}>{&segment.text}</span>
                }
            })}
        </>
    }
}