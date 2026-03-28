use pulldown_cmark::HeadingLevel;
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::markdown::HeadingInfo;

pub fn render(frame: &mut Frame, area: Rect, headings: &[HeadingInfo], selected: usize) {
    if headings.is_empty() {
        return;
    }

    let max_text_width = headings
        .iter()
        .map(|h| indent_for(h.level) + h.text.len())
        .max()
        .unwrap_or(20);

    let popup_width = (max_text_width + 6).min(area.width as usize - 4) as u16;
    let popup_height = (headings.len() as u16 + 4).min(area.height - 2);
    let popup = centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup);

    let visible_height = popup_height.saturating_sub(4) as usize;
    let scroll_offset = if selected >= visible_height {
        selected - visible_height + 1
    } else {
        0
    };

    let lines: Vec<Line> = headings
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_height)
        .map(|(i, h)| {
            let indent = " ".repeat(indent_for(h.level));
            let marker = if i == selected { "▸ " } else { "  " };
            let style = if i == selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                level_style(h.level)
            };
            Line::from(vec![
                Span::raw(format!("{indent}{marker}")),
                Span::styled(&h.text, style),
            ])
        })
        .collect();

    let block = Block::bordered()
        .title(" Table of Contents ")
        .border_style(Style::default().fg(Color::Cyan));

    let paragraph = Paragraph::new(lines).block(block);
    frame.render_widget(paragraph, popup);
}

fn indent_for(level: HeadingLevel) -> usize {
    match level {
        HeadingLevel::H1 => 0,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 4,
        HeadingLevel::H4 => 6,
        HeadingLevel::H5 => 8,
        HeadingLevel::H6 => 10,
    }
}

fn level_style(level: HeadingLevel) -> Style {
    match level {
        HeadingLevel::H1 => Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
        HeadingLevel::H2 => Style::default().fg(Color::Yellow),
        HeadingLevel::H3 => Style::default().fg(Color::Green),
        HeadingLevel::H4 => Style::default().fg(Color::Blue),
        HeadingLevel::H5 => Style::default().fg(Color::Magenta),
        HeadingLevel::H6 => Style::default().fg(Color::DarkGray),
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center);
    let [y] = vertical.areas(area);
    let [x] = horizontal.areas(y);
    x
}
