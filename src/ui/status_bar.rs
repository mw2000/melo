use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    filename: &str,
    scroll_offset: u16,
    content_height: usize,
    viewport_height: u16,
    search_query: Option<&str>,
) {
    if let Some(query) = search_query {
        let search_line = Line::from(vec![
            Span::styled(" /", Style::default().fg(Color::Yellow)),
            Span::raw(query),
            Span::styled("█", Style::default().fg(Color::Yellow)),
        ])
        .style(Style::default().bg(Color::DarkGray));

        frame.render_widget(search_line, area);
        return;
    }

    let current_line = scroll_offset as usize + 1;
    let total = content_height.max(1);
    let end_line = (scroll_offset as usize + viewport_height as usize).min(total);

    let pct = if total <= viewport_height as usize {
        100
    } else if scroll_offset == 0 {
        0
    } else {
        ((scroll_offset as usize * 100) / total.saturating_sub(viewport_height as usize)).min(100)
    };

    let left = Span::styled(format!(" {filename} "), Style::default().bold());
    let position = Span::raw(format!(" {current_line}-{end_line}/{total} "));
    let percentage = Span::styled(format!(" {pct}% "), Style::default().fg(Color::Cyan));
    let hint = Span::styled(" ? help ", Style::default().fg(Color::DarkGray));

    let padding_len = (area.width as usize)
        .saturating_sub(left.width() + position.width() + percentage.width() + hint.width());
    let padding = Span::raw(" ".repeat(padding_len));

    let bar = Line::from(vec![left, padding, position, percentage, hint])
        .style(Style::default().bg(Color::DarkGray));

    frame.render_widget(bar, area);
}
