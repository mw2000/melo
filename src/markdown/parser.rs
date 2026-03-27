use ratatui::text::{Line, Span, Text};
use tui_markdown::Options;

use super::theme::Theme;

pub struct MarkdownDocument {
    pub text: Text<'static>,
    #[allow(dead_code)]
    pub source: String,
    pub title: Option<String>,
}

pub fn parse(content: &str, theme: &Theme) -> MarkdownDocument {
    let options = Options::new(theme.clone());
    let text = tui_markdown::from_str_with_options(content, &options);
    let text = into_static(text);
    let title = extract_first_heading(content);

    MarkdownDocument {
        text,
        source: content.to_string(),
        title,
    }
}

fn into_static(text: Text<'_>) -> Text<'static> {
    let lines: Vec<Line<'static>> = text
        .lines
        .into_iter()
        .map(|line| {
            let alignment = line.alignment;
            let style = line.style;
            let spans: Vec<Span<'static>> = line
                .spans
                .into_iter()
                .map(|span| Span::styled(span.content.into_owned(), span.style))
                .collect();
            let mut new_line = Line::from(spans);
            new_line.alignment = alignment;
            new_line.style = style;
            new_line
        })
        .collect();
    Text::from(lines)
}

fn extract_first_heading(content: &str) -> Option<String> {
    content
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches("# ").to_string())
}
