use std::path::Path;

use ratatui::text::Text;

use super::renderer::{self, HeadingInfo, LinkInfo};
use super::theme::Theme;

pub struct MarkdownDocument {
    pub text: Text<'static>,
    #[allow(dead_code)]
    pub source: String,
    pub title: Option<String>,
    pub headings: Vec<HeadingInfo>,
    pub links: Vec<LinkInfo>,
}

pub fn parse(content: &str, theme: &Theme, base_dir: Option<&Path>) -> MarkdownDocument {
    let output = renderer::render(content, theme, base_dir);
    let title = output.headings.first().map(|h| h.text.clone());

    MarkdownDocument {
        text: output.text,
        source: content.to_string(),
        title,
        headings: output.headings,
        links: output.links,
    }
}
