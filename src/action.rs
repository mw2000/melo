/// Commands that the app can execute, produced by [`crate::input::InputMap`] or mouse events.
/// The `u16` on scroll variants is the number of lines to move.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Action {
    Quit,
    ScrollUp(u16),
    ScrollDown(u16),
    PageUp,
    PageDown,
    Top,
    Bottom,
    ToggleHelp,
    EnterSearch,
    SearchNext,
    SearchPrev,
    NextHeading,
    PrevHeading,
    ToggleToc,
    ToggleLinkPicker,
    OpenLink,
    GoBack,
}
