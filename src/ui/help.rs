use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

const BINDINGS: &[(&str, &str)] = &[
    ("j / ↓", "Scroll down"),
    ("k / ↑", "Scroll up"),
    ("Ctrl-d / PgDn", "Half-page down"),
    ("Ctrl-u / PgUp", "Half-page up"),
    ("g / Home", "Top"),
    ("G / End", "Bottom"),
    ("Tab / Shift-Tab", "Next / prev heading"),
    ("t", "Table of contents"),
    ("o", "Open link picker"),
    ("Backspace", "Go back (after link follow)"),
    ("/ ", "Search"),
    ("n / N", "Next / prev match"),
    ("?", "Toggle this help"),
    ("q / Esc", "Quit"),
];

/// Render a centered keybinding reference popup over the current content.
pub fn render(frame: &mut Frame, area: Rect) {
    let popup_width = 38_u16;
    let popup_height = (BINDINGS.len() as u16) + 4;
    let popup = centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup);

    let lines: Vec<Line> = BINDINGS
        .iter()
        .map(|(key, desc)| {
            Line::from(vec![
                Span::styled(
                    format!(" {key:<16}"),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("{desc} ")),
            ])
        })
        .collect();

    let block = Block::bordered()
        .title(" Keybindings ")
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup);
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [y] = vertical.areas(area);
    let [x] = horizontal.areas(y);
    x
}
