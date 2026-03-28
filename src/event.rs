use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind};

use crate::action::Action;

/// Thin abstraction over crossterm events, filtering to only key presses
/// and mouse scroll.
pub enum AppEvent {
    Key(KeyEvent),
    /// Mouse scroll pre-converted to an [`Action`] (ScrollUp/ScrollDown by 3 lines).
    Scroll(Action),
}

/// Non-blocking event poll. Returns `None` on timeout or ignored events.
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
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
