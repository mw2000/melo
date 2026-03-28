use ratatui::style::{Color, Modifier, Style};

/// Visual styles for each markdown element. The renderer reads these when converting
/// pulldown-cmark events to styled spans. Use named constructors ([`Theme::dark`],
/// [`Theme::light`], [`Theme::ocean`]) or [`Theme::from_name`] for built-in variants.
#[derive(Debug, Clone)]
pub struct Theme {
    pub syntect_theme: String,
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

impl Theme {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "dark" => Some(Self::dark()),
            "light" => Some(Self::light()),
            "ocean" => Some(Self::ocean()),
            _ => None,
        }
    }

    pub fn available_themes() -> &'static [&'static str] {
        &["dark", "light", "ocean"]
    }

    pub fn dark() -> Self {
        Self {
            syntect_theme: "base16-ocean.dark".into(),
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

    pub fn light() -> Self {
        Self {
            syntect_theme: "InspiredGitHub".into(),
            h1: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(36, 41, 46))
                .add_modifier(Modifier::BOLD),
            h2: Style::default()
                .fg(Color::Rgb(0, 92, 197))
                .add_modifier(Modifier::BOLD),
            h3: Style::default()
                .fg(Color::Rgb(106, 115, 125))
                .add_modifier(Modifier::BOLD),
            h4: Style::default()
                .fg(Color::Rgb(111, 66, 193))
                .add_modifier(Modifier::BOLD),
            h5: Style::default()
                .fg(Color::Rgb(227, 98, 9))
                .add_modifier(Modifier::BOLD),
            h6: Style::default()
                .fg(Color::Rgb(150, 152, 154))
                .add_modifier(Modifier::BOLD),
            code_inline: Style::default()
                .fg(Color::Rgb(36, 41, 46))
                .bg(Color::Rgb(235, 236, 237)),
            link: Style::default()
                .fg(Color::Rgb(0, 92, 197))
                .add_modifier(Modifier::UNDERLINED),
            blockquote: Style::default().fg(Color::Rgb(106, 115, 125)),
            table_header: Style::default()
                .fg(Color::Rgb(36, 41, 46))
                .add_modifier(Modifier::BOLD),
            table_border: Style::default().fg(Color::Rgb(209, 213, 218)),
            hr: Style::default().fg(Color::Rgb(209, 213, 218)),
        }
    }

    pub fn ocean() -> Self {
        Self {
            syntect_theme: "base16-ocean.dark".into(),
            h1: Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(0, 95, 115))
                .add_modifier(Modifier::BOLD),
            h2: Style::default()
                .fg(Color::Rgb(95, 215, 175))
                .add_modifier(Modifier::BOLD),
            h3: Style::default()
                .fg(Color::Rgb(135, 206, 235))
                .add_modifier(Modifier::BOLD),
            h4: Style::default()
                .fg(Color::Rgb(0, 175, 175))
                .add_modifier(Modifier::BOLD),
            h5: Style::default()
                .fg(Color::Rgb(150, 200, 180))
                .add_modifier(Modifier::BOLD),
            h6: Style::default()
                .fg(Color::Rgb(100, 130, 140))
                .add_modifier(Modifier::BOLD),
            code_inline: Style::default()
                .fg(Color::Rgb(180, 210, 200))
                .bg(Color::Rgb(30, 50, 55)),
            link: Style::default()
                .fg(Color::Rgb(95, 215, 215))
                .add_modifier(Modifier::UNDERLINED),
            blockquote: Style::default().fg(Color::Rgb(100, 150, 150)),
            table_header: Style::default()
                .fg(Color::Rgb(95, 215, 175))
                .add_modifier(Modifier::BOLD),
            table_border: Style::default().fg(Color::Rgb(70, 100, 110)),
            hr: Style::default().fg(Color::Rgb(70, 100, 110)),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
