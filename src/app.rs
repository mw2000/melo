use std::path::PathBuf;
use std::time::Duration;

use color_eyre::{eyre::eyre, Result};

use crate::action::Action;
use crate::event::{self, AppEvent};
use crate::input::InputMap;
use crate::markdown::{self, MarkdownDocument, Theme};
use crate::terminal::TerminalGuard;
use crate::ui::{self, Viewport};

/// Active interaction mode. Determines how key events are routed.
#[derive(Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    /// Captures keystrokes into `search_query`; Enter commits, Esc cancels.
    Search,
    /// Any keypress returns to Normal.
    Help,
}

/// Top-level application state. Owns the parsed document, viewport, keybindings,
/// and search state. Constructed via [`AppBuilder`].
pub struct App {
    pub document: MarkdownDocument,
    pub viewport: Viewport,
    pub input_map: InputMap,
    #[allow(dead_code)]
    pub theme: Theme,
    pub filename: String,
    mode: Mode,
    search_query: String,
    search_matches: Vec<usize>,
    search_index: usize,
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

    pub fn is_searching(&self) -> bool {
        self.mode == Mode::Search
    }

    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    pub fn show_help(&self) -> bool {
        self.mode == Mode::Help
    }

    /// Main event loop: render → poll → dispatch → repeat at ~60fps (16ms poll timeout).
    /// Recomputes wrapped content height each frame so the scrollbar stays accurate on resize.
    pub fn run(&mut self) -> Result<()> {
        let mut guard = TerminalGuard::new()?;

        loop {
            let size = guard.terminal.size()?;
            let viewport_height = size.height.saturating_sub(3);
            let inner_width = size.width.saturating_sub(2);
            self.viewport
                .update_wrapped_height(&self.document.text, inner_width);
            self.viewport.clamp_scroll(viewport_height);

            guard.terminal.draw(|frame| {
                ui::render(frame, self);
            })?;

            if let Some(event) = event::poll(Duration::from_millis(16))? {
                match event {
                    AppEvent::Key(key) => self.handle_key(key, viewport_height),
                    AppEvent::Scroll(action) => self.handle_action(action, viewport_height),
                    AppEvent::Resize(_, _) => {}
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: crossterm::event::KeyEvent, viewport_height: u16) {
        use crossterm::event::KeyCode;

        match self.mode {
            Mode::Normal => {
                if let Some(action) = self.input_map.resolve(&key) {
                    self.handle_action(action, viewport_height);
                }
            }
            Mode::Search => match key.code {
                KeyCode::Enter => {
                    self.update_search();
                    self.mode = Mode::Normal;
                    if !self.search_matches.is_empty() {
                        self.jump_to_match();
                    }
                }
                KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                KeyCode::Backspace => {
                    self.search_query.pop();
                }
                KeyCode::Char(c) => {
                    self.search_query.push(c);
                }
                _ => {}
            },
            Mode::Help => {
                self.mode = Mode::Normal;
            }
        }
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
            Action::ToggleHelp => {
                self.mode = if self.mode == Mode::Help {
                    Mode::Normal
                } else {
                    Mode::Help
                };
            }
            Action::EnterSearch => {
                self.search_query.clear();
                self.search_matches.clear();
                self.search_index = 0;
                self.mode = Mode::Search;
            }
            Action::SearchNext => {
                if !self.search_matches.is_empty() {
                    self.search_index = (self.search_index + 1) % self.search_matches.len();
                    self.jump_to_match();
                }
            }
            Action::SearchPrev => {
                if !self.search_matches.is_empty() {
                    self.search_index = if self.search_index == 0 {
                        self.search_matches.len() - 1
                    } else {
                        self.search_index - 1
                    };
                    self.jump_to_match();
                }
            }
        }
    }

    /// Rebuild `search_matches` with line indices where `search_query` appears (case-insensitive).
    fn update_search(&mut self) {
        self.search_matches.clear();
        self.search_index = 0;

        if self.search_query.is_empty() {
            return;
        }

        let query_lower = self.search_query.to_lowercase();
        for (i, line) in self.document.text.lines.iter().enumerate() {
            let line_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
            if line_text.to_lowercase().contains(&query_lower) {
                self.search_matches.push(i);
            }
        }
    }

    fn jump_to_match(&mut self) {
        if let Some(&line) = self.search_matches.get(self.search_index) {
            self.viewport.scroll_to_line(line as u16);
        }
    }
}

/// Builder for [`App`]. Supply either a file path or raw content string, then
/// optionally override the theme and keybindings before calling [`build()`](AppBuilder::build).
#[derive(Default)]
pub struct AppBuilder {
    file: Option<PathBuf>,
    content: Option<(String, String)>,
    theme: Option<Theme>,
    input_map: Option<InputMap>,
}

impl AppBuilder {
    pub fn file(mut self, path: PathBuf) -> Self {
        self.file = Some(path);
        self
    }

    pub fn content(mut self, text: String, filename: String) -> Self {
        self.content = Some((text, filename));
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
        let (raw_content, filename) = match (self.file, self.content) {
            (Some(file), _) => {
                let content = std::fs::read_to_string(&file)?;
                let name = file.display().to_string();
                (content, name)
            }
            (None, Some((text, name))) => (text, name),
            (None, None) => return Err(eyre!("either a file path or content is required")),
        };

        let theme = self.theme.unwrap_or_default();
        let input_map = self.input_map.unwrap_or_else(InputMap::vim);
        let document = markdown::parse(&raw_content, &theme);
        let content_height = document.text.height();

        Ok(App {
            document,
            viewport: Viewport::new(content_height),
            input_map,
            theme,
            filename,
            mode: Mode::default(),
            search_query: String::new(),
            search_matches: Vec::new(),
            search_index: 0,
            should_quit: false,
        })
    }
}
