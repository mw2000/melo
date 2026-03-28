use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};

/// Scrollable content viewport. Tracks scroll position and content height (in wrapped lines).
/// Renders the markdown as a bordered [`Paragraph`] with word-wrap and a vertical scrollbar.
pub struct Viewport {
    scroll_offset: u16,
    content_height: usize,
}

impl Viewport {
    pub fn new(content_height: usize) -> Self {
        Self {
            scroll_offset: 0,
            content_height,
        }
    }

    /// Recompute `content_height` using [`Paragraph::line_count`] to account for word-wrap.
    /// Must be called each frame (or on resize) before [`clamp_scroll`](Viewport::clamp_scroll).
    pub fn update_wrapped_height(&mut self, text: &Text, inner_width: u16) {
        let paragraph = Paragraph::new(text.clone()).wrap(Wrap { trim: false });
        self.content_height = paragraph.line_count(inner_width);
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_add(amount);
    }

    pub fn page_up(&mut self, viewport_height: u16) {
        self.scroll_up(viewport_height / 2);
    }

    pub fn page_down(&mut self, viewport_height: u16) {
        self.scroll_down(viewport_height / 2);
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    pub fn scroll_to_bottom(&mut self, viewport_height: u16) {
        self.scroll_offset = self.max_scroll(viewport_height);
    }

    pub fn scroll_offset(&self) -> u16 {
        self.scroll_offset
    }

    pub fn content_height(&self) -> usize {
        self.content_height
    }

    pub fn scroll_to_line(&mut self, line: u16) {
        self.scroll_offset = line;
    }

    /// Maximum scroll offset: `content_height - viewport_height` (clamped to 0).
    fn max_scroll(&self, viewport_height: u16) -> u16 {
        (self.content_height as u16).saturating_sub(viewport_height)
    }

    pub fn clamp_scroll(&mut self, viewport_height: u16) {
        let max = self.max_scroll(viewport_height);
        self.scroll_offset = self.scroll_offset.min(max);
    }

    pub fn render(&self, text: &Text<'static>, area: Rect, frame: &mut Frame, title: &str) {
        let block = Block::bordered()
            .title(format!(" {} ", title))
            .border_style(Style::default().fg(Color::DarkGray));

        let paragraph = Paragraph::new(text.clone())
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.scroll_offset, 0));

        frame.render_widget(paragraph, area);

        let inner_height = area.height.saturating_sub(2) as usize;
        let max_scroll = self.content_height.saturating_sub(inner_height);
        if max_scroll > 0 {
            let mut scrollbar_state =
                ScrollbarState::new(max_scroll + 1).position(self.scroll_offset as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        }
    }
}
