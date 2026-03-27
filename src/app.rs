use std::path::PathBuf;
use std::time::Duration;

use color_eyre::{eyre::eyre, Result};

use crate::action::Action;
use crate::event::{self, AppEvent};
use crate::input::InputMap;
use crate::markdown::{self, MarkdownDocument, Theme};
use crate::terminal::TerminalGuard;
use crate::ui::{self, Viewport};

pub struct App {
    pub document: MarkdownDocument,
    pub viewport: Viewport,
    pub input_map: InputMap,
    #[allow(dead_code)]
    pub theme: Theme,
    pub filename: String,
    should_quit: bool,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn display_title(&self) -> String {
        match &self.document.title {
            Some(title) => format!("{} — {}", self.filename, title),
            None => self.filename.clone(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut guard = TerminalGuard::new()?;

        loop {
            let viewport_height = guard.terminal.size()?.height.saturating_sub(2);
            self.viewport.clamp_scroll(viewport_height);

            guard.terminal.draw(|frame| {
                ui::render(frame, self);
            })?;

            if let Some(event) = event::poll(Duration::from_millis(16))? {
                match event {
                    AppEvent::Key(key) => {
                        if let Some(action) = self.input_map.resolve(&key) {
                            self.handle_action(action, viewport_height);
                        }
                    }
                    AppEvent::Resize(_, _) => {}
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_action(&mut self, action: Action, viewport_height: u16) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::ScrollUp(n) => self.viewport.scroll_up(n),
            Action::ScrollDown(n) => self.viewport.scroll_down(n),
            Action::PageUp => self.viewport.page_up(viewport_height),
            Action::PageDown => self.viewport.page_down(viewport_height),
            Action::Top => self.viewport.scroll_to_top(),
            Action::Bottom => self.viewport.scroll_to_bottom(viewport_height),
        }
    }
}

#[derive(Default)]
pub struct AppBuilder {
    file: Option<PathBuf>,
    theme: Option<Theme>,
    input_map: Option<InputMap>,
}

impl AppBuilder {
    pub fn file(mut self, path: PathBuf) -> Self {
        self.file = Some(path);
        self
    }

    #[allow(dead_code)]
    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    #[allow(dead_code)]
    pub fn input_map(mut self, map: InputMap) -> Self {
        self.input_map = Some(map);
        self
    }

    pub fn build(self) -> Result<App> {
        let file = self.file.ok_or_else(|| eyre!("file path is required"))?;
        let content = std::fs::read_to_string(&file)?;
        let theme = self.theme.unwrap_or_default();
        let input_map = self.input_map.unwrap_or_else(InputMap::vim);
        let document = markdown::parse(&content, &theme);
        let content_height = document.text.height();

        Ok(App {
            document,
            viewport: Viewport::new(content_height),
            input_map,
            theme,
            filename: file.display().to_string(),
            should_quit: false,
        })
    }
}
