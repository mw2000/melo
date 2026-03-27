mod help;
mod status_bar;
mod viewport;

pub use viewport::Viewport;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    let viewport_area = chunks[0];
    let status_area = chunks[1];

    let inner_height = viewport_area.height.saturating_sub(2);

    app.viewport.render(
        &app.document.text,
        viewport_area,
        frame,
        &app.display_title(),
    );

    let search_query = if app.is_searching() {
        Some(app.search_query())
    } else {
        None
    };

    status_bar::render(
        frame,
        status_area,
        &app.filename,
        app.viewport.scroll_offset(),
        app.viewport.content_height(),
        inner_height,
        search_query,
    );

    if app.show_help() {
        help::render(frame, frame.area());
    }
}
