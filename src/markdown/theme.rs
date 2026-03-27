use ratatui::style::{Color, Modifier, Style};
use tui_markdown::StyleSheet;

#[derive(Debug, Clone)]
pub struct Theme {
    pub h1: Style,
    pub h2: Style,
    pub h3: Style,
    pub h4: Style,
    pub h5: Style,
    pub h6: Style,
    pub code: Style,
    pub link: Style,
    pub blockquote: Style,
    pub heading_meta: Style,
    pub metadata_block: Style,
}

impl StyleSheet for Theme {
    fn heading(&self, level: u8) -> Style {
        match level {
            1 => self.h1,
            2 => self.h2,
            3 => self.h3,
            4 => self.h4,
            5 => self.h5,
            _ => self.h6,
        }
    }

    fn code(&self) -> Style {
        self.code
    }

    fn link(&self) -> Style {
        self.link
    }

    fn blockquote(&self) -> Style {
        self.blockquote
    }

    fn heading_meta(&self) -> Style {
        self.heading_meta
    }

    fn metadata_block(&self) -> Style {
        self.metadata_block
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            h1: Style::default()
                .fg(Color::Cyan)
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
            code: Style::default().fg(Color::White).bg(Color::DarkGray),
            link: Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::UNDERLINED),
            blockquote: Style::default().fg(Color::Yellow),
            heading_meta: Style::default().fg(Color::DarkGray),
            metadata_block: Style::default().fg(Color::LightYellow),
        }
    }
}

#[allow(dead_code)]
pub struct ThemeBuilder {
    theme: Theme,
}

impl Theme {
    #[allow(dead_code)]
    pub fn builder() -> ThemeBuilder {
        ThemeBuilder {
            theme: Theme::default(),
        }
    }
}

#[allow(dead_code)]
impl ThemeBuilder {
    pub fn h1(mut self, style: Style) -> Self {
        self.theme.h1 = style;
        self
    }

    pub fn h2(mut self, style: Style) -> Self {
        self.theme.h2 = style;
        self
    }

    pub fn h3(mut self, style: Style) -> Self {
        self.theme.h3 = style;
        self
    }

    pub fn h4(mut self, style: Style) -> Self {
        self.theme.h4 = style;
        self
    }

    pub fn h5(mut self, style: Style) -> Self {
        self.theme.h5 = style;
        self
    }

    pub fn h6(mut self, style: Style) -> Self {
        self.theme.h6 = style;
        self
    }

    pub fn code(mut self, style: Style) -> Self {
        self.theme.code = style;
        self
    }

    pub fn link(mut self, style: Style) -> Self {
        self.theme.link = style;
        self
    }

    pub fn blockquote(mut self, style: Style) -> Self {
        self.theme.blockquote = style;
        self
    }

    pub fn heading_meta(mut self, style: Style) -> Self {
        self.theme.heading_meta = style;
        self
    }

    pub fn metadata_block(mut self, style: Style) -> Self {
        self.theme.metadata_block = style;
        self
    }

    pub fn build(self) -> Theme {
        self.theme
    }
}
