use ratatui::text::Text;

use super::{renderer, theme::Theme};

/// The result of parsing a markdown string. Contains the rendered ratatui [`Text`],
/// the original source (for search), and the first `# heading` if present.
pub struct MarkdownDocument {
    pub text: Text<'static>,
    #[allow(dead_code)]
    pub source: String,
    pub title: Option<String>,
}

/// Parse raw markdown into a [`MarkdownDocument`]. Delegates rendering to
/// [`renderer::render`] and extracts the first ATX heading for the title bar.
pub fn parse(content: &str, theme: &Theme) -> MarkdownDocument {
    let text = renderer::render(content, theme);
    let title = extract_first_heading(content);

    MarkdownDocument {
        text,
        source: content.to_string(),
        title,
    }
}

fn extract_first_heading(content: &str) -> Option<String> {
    content
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches("# ").to_string())
}
