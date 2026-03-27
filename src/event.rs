use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};

use crate::action::Action;

pub enum AppEvent {
    Key(KeyEvent),
    Scroll(Action),
    #[allow(dead_code)]
    Resize(u16, u16),
}

pub fn poll(timeout: Duration) -> Result<Option<AppEvent>> {
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => Ok(Some(AppEvent::Key(key))),
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::ScrollUp,
                ..
            }) => Ok(Some(AppEvent::Scroll(Action::ScrollUp(3)))),
            Event::Mouse(MouseEvent {
                kind: MouseEventKind::ScrollDown,
                ..
            }) => Ok(Some(AppEvent::Scroll(Action::ScrollDown(3)))),
            Event::Resize(w, h) => Ok(Some(AppEvent::Resize(w, h))),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
