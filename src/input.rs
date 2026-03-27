use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;

pub struct InputMap {
    bindings: HashMap<(KeyCode, KeyModifiers), Action>,
}

pub struct InputMapBuilder {
    bindings: HashMap<(KeyCode, KeyModifiers), Action>,
}

impl InputMap {
    pub fn builder() -> InputMapBuilder {
        InputMapBuilder {
            bindings: HashMap::new(),
        }
    }

    pub fn vim() -> Self {
        Self::builder()
            .bind(KeyCode::Char('q'), KeyModifiers::NONE, Action::Quit)
            .bind(KeyCode::Esc, KeyModifiers::NONE, Action::Quit)
            .bind(KeyCode::Char('c'), KeyModifiers::CONTROL, Action::Quit)
            .bind(
                KeyCode::Char('j'),
                KeyModifiers::NONE,
                Action::ScrollDown(1),
            )
            .bind(KeyCode::Down, KeyModifiers::NONE, Action::ScrollDown(1))
            .bind(KeyCode::Char('k'), KeyModifiers::NONE, Action::ScrollUp(1))
            .bind(KeyCode::Up, KeyModifiers::NONE, Action::ScrollUp(1))
            .bind(KeyCode::Char('d'), KeyModifiers::CONTROL, Action::PageDown)
            .bind(KeyCode::PageDown, KeyModifiers::NONE, Action::PageDown)
            .bind(KeyCode::Char('u'), KeyModifiers::CONTROL, Action::PageUp)
            .bind(KeyCode::PageUp, KeyModifiers::NONE, Action::PageUp)
            .bind(KeyCode::Char('g'), KeyModifiers::NONE, Action::Top)
            .bind(KeyCode::Home, KeyModifiers::NONE, Action::Top)
            .bind(KeyCode::Char('G'), KeyModifiers::SHIFT, Action::Bottom)
            .bind(KeyCode::Char('G'), KeyModifiers::NONE, Action::Bottom)
            .bind(KeyCode::End, KeyModifiers::NONE, Action::Bottom)
            .bind(KeyCode::Char('?'), KeyModifiers::NONE, Action::ToggleHelp)
            .bind(KeyCode::Char('?'), KeyModifiers::SHIFT, Action::ToggleHelp)
            .bind(KeyCode::Char('/'), KeyModifiers::NONE, Action::EnterSearch)
            .bind(KeyCode::Char('n'), KeyModifiers::NONE, Action::SearchNext)
            .bind(KeyCode::Char('N'), KeyModifiers::SHIFT, Action::SearchPrev)
            .bind(KeyCode::Char('N'), KeyModifiers::NONE, Action::SearchPrev)
            .build()
    }

    pub fn resolve(&self, key: &KeyEvent) -> Option<Action> {
        self.bindings.get(&(key.code, key.modifiers)).copied()
    }
}

impl InputMapBuilder {
    pub fn bind(mut self, code: KeyCode, modifiers: KeyModifiers, action: Action) -> Self {
        self.bindings.insert((code, modifiers), action);
        self
    }

    #[allow(dead_code)]
    pub fn unbind(mut self, code: KeyCode, modifiers: KeyModifiers) -> Self {
        self.bindings.remove(&(code, modifiers));
        self
    }

    pub fn build(self) -> InputMap {
        InputMap {
            bindings: self.bindings,
        }
    }
}
