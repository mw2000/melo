use std::path::Path;

use image::imageops::FilterType;
use image::ImageReader;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

const MAX_IMAGE_WIDTH: u32 = 72;

/// Load a local image file and render it as halfblock-encoded ratatui Lines.
/// Each terminal cell encodes two vertical pixels using `▄` with fg (bottom) and bg (top).
/// Returns `None` if the file can't be loaded or decoded.
pub fn render_image(path: &Path) -> Option<Vec<Line<'static>>> {
    let img = ImageReader::open(path)
        .ok()?
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;
    let img = img.to_rgba8();

    let (w, h) = (img.width(), img.height());
    if w == 0 || h == 0 {
        return None;
    }

    let scale = if w > MAX_IMAGE_WIDTH {
        MAX_IMAGE_WIDTH as f64 / w as f64
    } else {
        1.0
    };
    // Terminal cells are ~2:1 aspect ratio (tall), so halve height
    let new_w = (w as f64 * scale) as u32;
    let new_h = (h as f64 * scale * 0.5) as u32;
    let new_h = if !new_h.is_multiple_of(2) {
        new_h + 1
    } else {
        new_h
    };

    let resized = image::imageops::resize(&img, new_w, new_h, FilterType::Triangle);

    let mut lines = Vec::new();

    let mut y = 0u32;
    while y < resized.height() {
        let mut spans = Vec::new();
        for x in 0..resized.width() {
            let top = resized.get_pixel(x, y);
            let bottom = if y + 1 < resized.height() {
                *resized.get_pixel(x, y + 1)
            } else {
                *top
            };

            let bg = Color::Rgb(top[0], top[1], top[2]);
            let fg = Color::Rgb(bottom[0], bottom[1], bottom[2]);

            spans.push(Span::styled("▄", Style::default().fg(fg).bg(bg)));
        }
        lines.push(Line::from(spans));
        y += 2;
    }

    Some(lines)
}
