use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};

pub enum AppEvent {
    Key(KeyEvent),
    #[allow(dead_code)]
    Resize(u16, u16),
}

pub fn poll(timeout: Duration) -> Result<Option<AppEvent>> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => Ok(Some(AppEvent::Key(key))),
            Event::Resize(w, h) => Ok(Some(AppEvent::Resize(w, h))),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
