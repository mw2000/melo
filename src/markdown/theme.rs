use ratatui::style::{Color, Modifier, Style};

/// Visual styles for each markdown element. The renderer reads these when converting
/// pulldown-cmark events to styled spans. Code block syntax highlighting comes from
/// syntect's "base16-ocean.dark" theme and is not controlled by these fields.
#[derive(Debug, Clone)]
pub struct Theme {
    pub h1: Style,
    pub h2: Style,
    pub h3: Style,
    pub h4: Style,
    pub h5: Style,
    pub h6: Style,
    /// Backtick-wrapped inline code, not fenced code blocks.
    pub code_inline: Style,
    pub link: Style,
    pub blockquote: Style,
    pub table_header: Style,
    pub table_border: Style,
    pub hr: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            h1: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(0, 135, 175))
                .add_modifier(Modifier::BOLD),
            h2: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            h3: Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            h4: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            h5: Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            h6: Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
            code_inline: Style::default()
                .fg(Color::LightYellow)
                .bg(Color::Rgb(50, 50, 50)),

            link: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::UNDERLINED),
            blockquote: Style::default().fg(Color::Rgb(150, 150, 150)),
            table_header: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            table_border: Style::default().fg(Color::DarkGray),
            hr: Style::default().fg(Color::DarkGray),
        }
    }
}
