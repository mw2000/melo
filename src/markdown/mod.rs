//! Markdown parsing and rendering pipeline.
//!
//! Flow: raw markdown string → [`pulldown_cmark::Parser`] → [`renderer`] → styled [`ratatui::text::Text`].
//! The [`renderer`] handles headings, code blocks (syntax-highlighted via syntect),
//! tables (box-drawing chars), lists, blockquotes, links, and inline formatting.

mod parser;
mod renderer;
mod theme;

pub use parser::{parse, MarkdownDocument};
pub use theme::Theme;
