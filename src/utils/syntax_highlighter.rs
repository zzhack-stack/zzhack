// Syntax highlighting utilities using syntect
// Provides syntax highlighting for code blocks in markdown

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxSet, SyntaxReference};

/// Syntax highlighter that provides syntax highlighting for code blocks
pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    /// Create a new syntax highlighter with default syntax and theme sets
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    /// Highlight code with the specified language
    /// 
    /// # Arguments
    /// * `code` - The code string to highlight
    /// * `language` - The programming language (e.g., "rust", "javascript", "python")
    /// 
    /// # Returns
    /// HTML string with syntax highlighting applied
    pub fn highlight_code(&self, code: &str, language: &str) -> String {
        // Get the syntax for the specified language
        let syntax = self.find_syntax_by_name(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        // Use a dark theme that works well with terminal aesthetics
        let theme = &self.theme_set.themes["base16-ocean.dark"];
        
        // Use the simpler API for highlighting
        match highlighted_html_for_string(code, &self.syntax_set, syntax, theme) {
            Ok(html) => html,
            Err(_) => {
                // Fallback to plain text if highlighting fails
                html_escape(code)
            }
        }
    }

    /// Find syntax by language name with fallback logic
    fn find_syntax_by_name(&self, language: &str) -> Option<&SyntaxReference> {
        // First try direct name match
        if let Some(syntax) = self.syntax_set.find_syntax_by_name(language) {
            return Some(syntax);
        }

        // Try common language aliases
        let normalized_lang = language.to_lowercase();
        match normalized_lang.as_str() {
            "rs" | "rust" => self.syntax_set.find_syntax_by_name("Rust"),
            "js" | "javascript" => self.syntax_set.find_syntax_by_name("JavaScript"),
            "ts" | "typescript" => self.syntax_set.find_syntax_by_name("TypeScript"),
            "py" | "python" => self.syntax_set.find_syntax_by_name("Python"),
            "sh" | "bash" | "shell" => self.syntax_set.find_syntax_by_name("Bash"),
            "html" => self.syntax_set.find_syntax_by_name("HTML"),
            "css" => self.syntax_set.find_syntax_by_name("CSS"),
            "json" => self.syntax_set.find_syntax_by_name("JSON"),
            "yaml" | "yml" => self.syntax_set.find_syntax_by_name("YAML"),
            "toml" => self.syntax_set.find_syntax_by_name("TOML"),
            "md" | "markdown" => self.syntax_set.find_syntax_by_name("Markdown"),
            "xml" => self.syntax_set.find_syntax_by_name("XML"),
            "sql" => self.syntax_set.find_syntax_by_name("SQL"),
            "c" => self.syntax_set.find_syntax_by_name("C"),
            "cpp" | "c++" => self.syntax_set.find_syntax_by_name("C++"),
            "java" => self.syntax_set.find_syntax_by_name("Java"),
            "go" => self.syntax_set.find_syntax_by_name("Go"),
            "php" => self.syntax_set.find_syntax_by_name("PHP"),
            "ruby" | "rb" => self.syntax_set.find_syntax_by_name("Ruby"),
            "swift" => self.syntax_set.find_syntax_by_name("Swift"),
            "kotlin" | "kt" => self.syntax_set.find_syntax_by_name("Kotlin"),
            "cs" | "csharp" => self.syntax_set.find_syntax_by_name("C#"),
            _ => {
                // Try extension-based lookup
                self.syntax_set.find_syntax_by_extension(&normalized_lang)
            }
        }
    }

    /// Get available theme names
    pub fn get_available_themes(&self) -> Vec<String> {
        self.theme_set.themes.keys().cloned().collect()
    }

    /// Get available syntax names  
    pub fn get_available_syntaxes(&self) -> Vec<String> {
        self.syntax_set.syntaxes()
            .iter()
            .map(|syntax| syntax.name.clone())
            .collect()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Escape HTML special characters
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_highlighter_creation() {
        let highlighter = SyntaxHighlighter::new();
        assert!(!highlighter.get_available_themes().is_empty());
        assert!(!highlighter.get_available_syntaxes().is_empty());
    }

    #[test]
    fn test_rust_code_highlighting() {
        let highlighter = SyntaxHighlighter::new();
        let rust_code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let highlighted = highlighter.highlight_code(rust_code, "rust");
        
        // Should contain some HTML tags for highlighting
        assert!(highlighted.contains("<span"));
    }

    #[test]
    fn test_unknown_language_fallback() {
        let highlighter = SyntaxHighlighter::new();
        let code = "some random text";
        let highlighted = highlighter.highlight_code(code, "unknown_language");
        
        // Should still return the code (plain text highlighting)
        assert!(highlighted.contains("some random text"));
    }
}