use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};

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

    #[allow(dead_code)]
    pub fn scroll_offset(&self) -> u16 {
        self.scroll_offset
    }

    #[allow(dead_code)]
    pub fn content_height(&self) -> usize {
        self.content_height
    }

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
        if self.content_height > inner_height {
            let mut scrollbar_state =
                ScrollbarState::new(self.content_height).position(self.scroll_offset as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
            frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
        }
    }
}
