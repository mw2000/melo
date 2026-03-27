mod parser;
mod theme;

pub use parser::{parse, MarkdownDocument};
pub use theme::Theme;
#[allow(unused_imports)]
pub use theme::ThemeBuilder;
