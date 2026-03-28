use ratatui::text::Text;

use super::{renderer, theme::Theme};

pub struct MarkdownDocument {
    pub text: Text<'static>,
    #[allow(dead_code)]
    pub source: String,
    pub title: Option<String>,
}

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
