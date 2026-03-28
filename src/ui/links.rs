use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::markdown::LinkInfo;

pub fn render(frame: &mut Frame, area: Rect, links: &[LinkInfo], selected: usize) {
    if links.is_empty() {
        return;
    }

    let max_width = links
        .iter()
        .map(|l| l.text.len() + l.url.len() + 5)
        .max()
        .unwrap_or(30);

    let popup_width = (max_width + 6).min(area.width as usize - 4) as u16;
    let popup_height = (links.len() as u16 + 4).min(area.height - 2);
    let popup = centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup);

    let visible_height = popup_height.saturating_sub(4) as usize;
    let scroll_offset = if selected >= visible_height {
        selected - visible_height + 1
    } else {
        0
    };

    let lines: Vec<Line> = links
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_height)
        .map(|(i, link)| {
            let marker = if i == selected { "▸ " } else { "  " };
            let text_style = if i == selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let url_style = Style::default().fg(Color::DarkGray);

            Line::from(vec![
                Span::raw(marker),
                Span::styled(&link.text, text_style),
                Span::styled(format!("  {}", link.url), url_style),
            ])
        })
        .collect();

    let title = format!(" Links ({}) ", links.len());
    let block = Block::bordered()
        .title(title)
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
