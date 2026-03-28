use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use color_eyre::{eyre::eyre, Result};

use crate::action::Action;
use crate::event::{self, AppEvent};
use crate::input::InputMap;
use crate::markdown::{self, MarkdownDocument, Theme};
use crate::terminal::TerminalGuard;
use crate::ui::{self, Viewport};
use crate::watcher::FileWatcher;

/// Active interaction mode. Determines how key events are routed.
#[derive(Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Normal,
    /// Captures keystrokes into `search_query`; Enter commits, Esc cancels.
    Search,
    Help,
    Toc,
    LinkPicker,
}

pub struct App {
    pub document: MarkdownDocument,
    pub viewport: Viewport,
    pub input_map: InputMap,
    pub theme: Theme,
    pub filename: String,
    pub file_path: Option<PathBuf>,
    mode: Mode,
    search_query: String,
    search_matches: Vec<usize>,
    search_index: usize,
    should_quit: bool,
    pub toc_index: usize,
    pub link_index: usize,
    file_stack: Vec<FileStackEntry>,
}

struct FileStackEntry {
    path: PathBuf,
    scroll_offset: u16,
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

    pub fn show_toc(&self) -> bool {
        self.mode == Mode::Toc
    }

    pub fn show_link_picker(&self) -> bool {
        self.mode == Mode::LinkPicker
    }

    pub fn run(&mut self) -> Result<()> {
        let mut guard = TerminalGuard::new()?;

        let (_watcher, watch_rx) = self.setup_watcher();

        loop {
            if let Some(rx) = &watch_rx
                && rx.try_recv().is_ok()
            {
                self.reload_file();
            }

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
                }
            }

            if self.should_quit {
                break;
            }
        }

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn setup_watcher(&self) -> (Option<FileWatcher>, Option<Receiver<()>>) {
        match &self.file_path {
            Some(path) => match FileWatcher::new(path.clone()) {
                Ok((watcher, rx)) => (Some(watcher), Some(rx)),
                Err(_) => (None, None),
            },
            None => (None, None),
        }
    }

    fn reload_file(&mut self) {
        let path = match &self.file_path {
            Some(p) => p,
            None => return,
        };

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return,
        };

        let scroll = self.viewport.scroll_offset();
        let base_dir = self.file_path.as_deref().and_then(|p| p.parent());
        let document = markdown::parse(&content, &self.theme, base_dir);
        let content_height = document.text.height();
        self.document = document;
        self.viewport = Viewport::new(content_height);
        self.viewport.scroll_to_line(scroll);
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
            Mode::Toc => match key.code {
                KeyCode::Esc | KeyCode::Char('t') | KeyCode::Char('q') => {
                    self.mode = Mode::Normal;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.document.headings.is_empty() {
                        self.toc_index = (self.toc_index + 1) % self.document.headings.len();
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if !self.document.headings.is_empty() {
                        self.toc_index = if self.toc_index == 0 {
                            self.document.headings.len() - 1
                        } else {
                            self.toc_index - 1
                        };
                    }
                }
                KeyCode::Enter => {
                    if let Some(heading) = self.document.headings.get(self.toc_index) {
                        self.viewport.scroll_to_line(heading.line as u16);
                        self.mode = Mode::Normal;
                    }
                }
                _ => {}
            },
            Mode::LinkPicker => match key.code {
                KeyCode::Esc | KeyCode::Char('o') | KeyCode::Char('q') => {
                    self.mode = Mode::Normal;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if !self.document.links.is_empty() {
                        self.link_index = (self.link_index + 1) % self.document.links.len();
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if !self.document.links.is_empty() {
                        self.link_index = if self.link_index == 0 {
                            self.document.links.len() - 1
                        } else {
                            self.link_index - 1
                        };
                    }
                }
                KeyCode::Enter => {
                    self.open_selected_link();
                }
                _ => {}
            },
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
            Action::NextHeading => {
                let current = self.viewport.scroll_offset() as usize;
                if let Some(h) = self.document.headings.iter().find(|h| h.line > current) {
                    self.viewport.scroll_to_line(h.line as u16);
                }
            }
            Action::PrevHeading => {
                let current = self.viewport.scroll_offset() as usize;
                if let Some(h) = self
                    .document
                    .headings
                    .iter()
                    .rev()
                    .find(|h| h.line < current)
                {
                    self.viewport.scroll_to_line(h.line as u16);
                }
            }
            Action::ToggleToc => {
                self.mode = if self.mode == Mode::Toc {
                    Mode::Normal
                } else {
                    self.toc_index = 0;
                    Mode::Toc
                };
            }
            Action::ToggleLinkPicker => {
                self.mode = if self.mode == Mode::LinkPicker {
                    Mode::Normal
                } else {
                    self.link_index = 0;
                    Mode::LinkPicker
                };
            }
            Action::GoBack => {
                self.navigate_back();
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

    fn open_selected_link(&mut self) {
        let url = match self.document.links.get(self.link_index) {
            Some(l) => l.url.clone(),
            None => return,
        };

        let is_local_md =
            !url.contains("://") && (url.ends_with(".md") || url.ends_with(".markdown"));

        if is_local_md {
            self.navigate_to_relative(&url);
        } else {
            let _ = open::that(&url);
        }
        self.mode = Mode::Normal;
    }

    fn navigate_to_relative(&mut self, relative_path: &str) {
        let base_dir = self
            .file_path
            .as_ref()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()));
        let target = match base_dir {
            Some(dir) => dir.join(relative_path),
            None => PathBuf::from(relative_path),
        };

        let content = match std::fs::read_to_string(&target) {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(path) = &self.file_path {
            self.file_stack.push(FileStackEntry {
                path: path.clone(),
                scroll_offset: self.viewport.scroll_offset(),
            });
        }

        let document = markdown::parse(&content, &self.theme, target.parent());
        let content_height = document.text.height();
        self.document = document;
        self.viewport = Viewport::new(content_height);
        self.filename = target.display().to_string();
        self.file_path = Some(target);
    }

    fn navigate_back(&mut self) {
        let entry = match self.file_stack.pop() {
            Some(e) => e,
            None => return,
        };

        let content = match std::fs::read_to_string(&entry.path) {
            Ok(c) => c,
            Err(_) => return,
        };

        let document = markdown::parse(&content, &self.theme, entry.path.parent());
        let content_height = document.text.height();
        self.document = document;
        self.viewport = Viewport::new(content_height);
        self.viewport.scroll_to_line(entry.scroll_offset);
        self.filename = entry.path.display().to_string();
        self.file_path = Some(entry.path);
    }
}

/// Builder for [`App`]. Supply either a file path or raw content string, then
/// optionally override the theme and keybindings before calling [`build()`](AppBuilder::build).
#[derive(Default)]
pub struct AppBuilder {
    file: Option<PathBuf>,
    content: Option<(String, String)>,
    theme: Option<Theme>,
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

    pub fn theme(mut self, theme: Theme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn build(self) -> Result<App> {
        let (raw_content, filename, file_path) = match (self.file, self.content) {
            (Some(file), _) => {
                let content = std::fs::read_to_string(&file)?;
                let name = file.display().to_string();
                (content, name, Some(file))
            }
            (None, Some((text, name))) => (text, name, None),
            (None, None) => return Err(eyre!("either a file path or content is required")),
        };

        let theme = self.theme.unwrap_or_default();
        let input_map = InputMap::vim();
        let base_dir = file_path.as_deref().and_then(|p| p.parent());
        let document = markdown::parse(&raw_content, &theme, base_dir);
        let content_height = document.text.height();

        Ok(App {
            document,
            viewport: Viewport::new(content_height),
            input_map,
            theme,
            filename,
            file_path,
            mode: Mode::default(),
            search_query: String::new(),
            search_matches: Vec::new(),
            search_index: 0,
            should_quit: false,
            toc_index: 0,
            link_index: 0,
            file_stack: Vec::new(),
        })
    }
}
