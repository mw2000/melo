#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    ScrollUp(u16),
    ScrollDown(u16),
    PageUp,
    PageDown,
    Top,
    Bottom,
}
