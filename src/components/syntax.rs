// Syntax Highlighting for Terminal Input
// This module handles syntax highlighting for command input with support for strings and commands

use yew::prelude::*;

/// Parse and render syntax highlighted text segments
#[derive(Clone, PartialEq)]
pub struct SyntaxSegment {
    pub text: String,
    pub segment_type: SegmentType,
}

#[derive(Clone, PartialEq)]
pub enum SegmentType {
    ValidCommand,
    InvalidCommand,
    String,
    Text,
}

/// Parse command text into syntax segments with dynamic command checking
pub fn parse_syntax_segments(text: &str, valid_commands: &[String]) -> Vec<SyntaxSegment> {
    if text.is_empty() {
        return vec![];
    }

    let mut segments = Vec::new();
    let mut chars = text.chars().peekable();
    let mut current_text = String::new();
    let mut is_first_word = true;
    let mut in_string = false;
    let mut string_char = '\0';

    while let Some(ch) = chars.next() {
        match ch {
            '\'' | '"' if !in_string => {
                // Start of string - save any pending text first
                if !current_text.is_empty() {
                    let segment_type = if is_first_word {
                        let parts: Vec<&str> = current_text.split_whitespace().collect();
                        if !parts.is_empty() {
                            let command = parts[0];
                            // Use dynamic command validation
                            let is_valid = valid_commands.contains(&command.to_string());
                            if is_valid {
                                SegmentType::ValidCommand
                            } else {
                                SegmentType::InvalidCommand
                            }
                        } else {
                            SegmentType::Text
                        }
                    } else {
                        SegmentType::Text
                    };
                    segments.push(SyntaxSegment {
                        text: current_text.clone(),
                        segment_type,
                    });
                    current_text.clear();
                    is_first_word = false;
                }

                // Start string
                in_string = true;
                string_char = ch;
                current_text.push(ch);
            }
            '\'' | '"' if in_string && ch == string_char => {
                // End of string
                current_text.push(ch);
                segments.push(SyntaxSegment {
                    text: current_text.clone(),
                    segment_type: SegmentType::String,
                });
                current_text.clear();
                in_string = false;
                string_char = '\0';
            }
            ' ' if !in_string => {
                // Space outside string - finish current segment
                if !current_text.is_empty() {
                    let segment_type = if is_first_word {
                        let parts: Vec<&str> = current_text.split_whitespace().collect();
                        if !parts.is_empty() {
                            let command = parts[0];
                            // Use dynamic command validation
                            let is_valid = valid_commands.contains(&command.to_string());
                            if is_valid {
                                SegmentType::ValidCommand
                            } else {
                                SegmentType::InvalidCommand
                            }
                        } else {
                            SegmentType::Text
                        }
                    } else {
                        SegmentType::Text
                    };
                    segments.push(SyntaxSegment {
                        text: current_text.clone(),
                        segment_type,
                    });
                    current_text.clear();
                    is_first_word = false;
                }
                current_text.push(ch);
            }
            _ => {
                current_text.push(ch);
            }
        }
    }

    // Add final segment
    if !current_text.is_empty() {
        let segment_type = if in_string {
            SegmentType::String
        } else if is_first_word {
            let parts: Vec<&str> = current_text.split_whitespace().collect();
            if !parts.is_empty() {
                let command = parts[0];
                // Use dynamic command validation instead of hardcoded matches
                let is_valid = valid_commands.contains(&command.to_string());
                if is_valid {
                    SegmentType::ValidCommand
                } else {
                    SegmentType::InvalidCommand
                }
            } else {
                SegmentType::Text
            }
        } else {
            SegmentType::Text
        };
        segments.push(SyntaxSegment {
            text: current_text,
            segment_type,
        });
    }

    segments
}

/// Generate syntax highlighted command display
pub fn render_command_with_syntax(command_text: &str, valid_commands: &[String]) -> Html {
    if command_text.is_empty() {
        return html! {};
    }

    let segments = parse_syntax_segments(command_text, valid_commands);
    
    html! {
        <>
            {for segments.iter().map(|segment| {
                let class = match segment.segment_type {
                    SegmentType::ValidCommand => "text-green-500 font-bold text-sm font-mono",
                    SegmentType::InvalidCommand => "text-terminal-error font-bold text-sm font-mono",
                    SegmentType::String => "text-yellow-400 text-sm font-mono",
                    SegmentType::Text => "text-terminal-text text-sm font-mono",
                };
                
                html! {
                    <span class={class}>{&segment.text}</span>
                }
            })}
        </>
    }
}

/// Render syntax segments for live input highlighting
pub fn render_syntax_segments(segments: &[SyntaxSegment]) -> Html {
    html! {
        <>
            {for segments.iter().map(|segment| {
                let class = match segment.segment_type {
                    SegmentType::ValidCommand => "text-green-500 font-bold",
                    SegmentType::InvalidCommand => "text-terminal-error font-bold", 
                    SegmentType::String => "text-yellow-400",
                    SegmentType::Text => "text-terminal-text",
                };
                
                html! {
                    <span class={class}>{&segment.text}</span>
                }
            })}
        </>
    }
}