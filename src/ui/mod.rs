mod viewport;

pub use viewport::Viewport;

use ratatui::Frame;

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    app.viewport
        .render(&app.document.text, area, frame, &app.display_title());
}
