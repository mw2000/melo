//! Converts pulldown-cmark events into styled ratatui [`Text`].
//!
//! The core type is [`Context`], a stateful walker that processes markdown events
//! sequentially and accumulates [`Line`]/[`Span`] output. Key rendering paths:
//! - **Headings**: H1 gets a padded line with background color; H2-H6 are bold + color.
//! - **Code blocks**: GitHub-style bordered boxes (╭╮╰╯) with syntect syntax highlighting.
//! - **Tables**: Box-drawing characters (┌┬┐│├┼┤└┴┘) with Unicode-width-aware columns.
//! - **Inline**: Bold, italic, strikethrough, links (with URL suffix), inline code.

use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

use super::theme::Theme;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

#[derive(Debug, Clone)]
pub struct HeadingInfo {
    pub line: usize,
    pub level: HeadingLevel,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct LinkInfo {
    pub url: String,
    pub text: String,
}

pub struct RenderOutput {
    pub text: Text<'static>,
    pub headings: Vec<HeadingInfo>,
    pub links: Vec<LinkInfo>,
}

pub fn render(content: &str, theme: &Theme, base_dir: Option<&Path>) -> RenderOutput {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let parser = Parser::new_ext(content, opts);
    let mut ctx = Context::new(theme, base_dir);

    for event in parser {
        ctx.push_event(event);
    }

    ctx.into_output()
}

/// Stateful markdown-to-ratatui converter. Walks pulldown-cmark events via [`push_event`](Context::push_event),
/// building up `lines`/`spans`. Nested formatting is tracked in `style_stack`; block-level
/// elements (code blocks, tables) buffer content in their own state structs until the
/// closing tag triggers a flush to `lines`.
struct Context<'a> {
    theme: &'a Theme,
    base_dir: Option<PathBuf>,
    lines: Vec<Line<'static>>,
    /// Spans being accumulated for the current line (flushed on newline/block boundary).
    spans: Vec<Span<'static>>,
    /// Nested inline styles (bold inside italic, etc). Merged via `Style::patch`.
    style_stack: Vec<Style>,

    list_stack: Vec<ListKind>,
    blockquote_depth: usize,
    heading: Option<HeadingLevel>,
    code_block: Option<CodeBlockState>,
    table: Option<TableState>,
    pending_link_url: Option<String>,
    pending_link_text: String,
    /// True when the current image was rendered inline as halfblocks (skip alt text / URL suffix).
    image_rendered: bool,
    /// True while inside a YAML front matter block — all content is discarded.
    in_metadata: bool,
    /// Tracks whether the next block element should emit a blank line separator.
    needs_newline: bool,

    headings: Vec<HeadingInfo>,
    links: Vec<LinkInfo>,
}

enum ListKind {
    Unordered,
    Ordered(u64),
}

/// Accumulates fenced code block content until the closing tag, then renders
/// the entire block at once with syntax highlighting and box borders.
struct CodeBlockState {
    lang: Option<String>,
    content: String,
}

/// Accumulates table rows/cells until the closing `</table>` tag, then renders
/// the full table with box-drawing borders and Unicode-width-aware column sizing.
struct TableState {
    rows: Vec<TableRow>,
    current_cells: Vec<Vec<Span<'static>>>,
    current_cell: Vec<Span<'static>>,
    in_header: bool,
}

struct TableRow {
    cells: Vec<Vec<Span<'static>>>,
    is_header: bool,
}

impl<'a> Context<'a> {
    fn new(theme: &'a Theme, base_dir: Option<&Path>) -> Self {
        Self {
            theme,
            base_dir: base_dir.map(Path::to_path_buf),
            lines: Vec::new(),
            spans: Vec::new(),
            style_stack: Vec::new(),
            list_stack: Vec::new(),
            blockquote_depth: 0,
            heading: None,
            code_block: None,
            table: None,
            pending_link_url: None,
            pending_link_text: String::new(),
            image_rendered: false,
            in_metadata: false,
            needs_newline: false,
            headings: Vec::new(),
            links: Vec::new(),
        }
    }

    fn current_style(&self) -> Style {
        let mut style = Style::default();
        for s in &self.style_stack {
            style = style.patch(*s);
        }
        style
    }

    fn flush_line(&mut self) {
        if !self.spans.is_empty() {
            self.lines.push(Line::from(std::mem::take(&mut self.spans)));
        }
    }

    fn push_blank(&mut self) {
        self.flush_line();
        self.lines.push(Line::default());
    }

    fn push_event(&mut self, event: Event) {
        if self.in_metadata {
            if matches!(event, Event::End(TagEnd::MetadataBlock(_))) {
                self.in_metadata = false;
            }
            return;
        }
        match event {
            Event::Start(Tag::MetadataBlock(_)) => {
                self.in_metadata = true;
            }
            Event::Start(tag) => self.start_tag(tag),
            Event::End(tag) => self.end_tag(tag),
            Event::Text(text) => self.text(&text),
            Event::Code(code) => self.inline_code(&code),
            Event::SoftBreak => self.soft_break(),
            Event::HardBreak => self.hard_break(),
            Event::Rule => self.rule(),
            Event::TaskListMarker(checked) => self.task_list_marker(checked),
            _ => {}
        }
    }

    fn start_tag(&mut self, tag: Tag) {
        match tag {
            Tag::Paragraph => {
                if self.needs_newline {
                    self.push_blank();
                }
            }
            Tag::Heading { level, .. } => {
                if self.needs_newline {
                    self.push_blank();
                }
                self.heading = Some(level);
            }
            Tag::BlockQuote(_) => {
                if self.blockquote_depth == 0 && self.needs_newline {
                    self.push_blank();
                }
                self.blockquote_depth += 1;
            }
            Tag::CodeBlock(kind) => {
                if self.needs_newline {
                    self.push_blank();
                }
                let lang = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let l = lang.trim().to_string();
                        if l.is_empty() {
                            None
                        } else {
                            Some(l)
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
                self.code_block = Some(CodeBlockState {
                    lang,
                    content: String::new(),
                });
            }
            Tag::List(start) => {
                if self.list_stack.is_empty() && self.needs_newline {
                    self.push_blank();
                }
                let kind = match start {
                    Some(n) => ListKind::Ordered(n),
                    None => ListKind::Unordered,
                };
                self.list_stack.push(kind);
            }
            Tag::Item => {
                let depth = self.list_stack.len().saturating_sub(1);
                let indent = "  ".repeat(depth);

                let marker = match self.list_stack.last_mut() {
                    Some(ListKind::Unordered) => "• ".to_string(),
                    Some(ListKind::Ordered(n)) => {
                        let m = format!("{}. ", n);
                        *n += 1;
                        m
                    }
                    None => "• ".to_string(),
                };

                self.spans.push(Span::raw(indent));
                self.spans
                    .push(Span::styled(marker, Style::default().fg(Color::DarkGray)));
            }
            Tag::Table(_) => {
                if self.needs_newline {
                    self.push_blank();
                }
                self.table = Some(TableState {
                    rows: Vec::new(),
                    current_cells: Vec::new(),
                    current_cell: Vec::new(),
                    in_header: false,
                });
            }
            Tag::TableHead => {
                if let Some(table) = &mut self.table {
                    table.in_header = true;
                    table.current_cells.clear();
                }
            }
            Tag::TableRow => {
                if let Some(table) = &mut self.table {
                    table.current_cells.clear();
                }
            }
            Tag::TableCell => {
                if let Some(table) = &mut self.table {
                    table.current_cell.clear();
                }
            }
            Tag::Emphasis => {
                self.style_stack
                    .push(Style::default().add_modifier(Modifier::ITALIC));
            }
            Tag::Strong => {
                self.style_stack
                    .push(Style::default().add_modifier(Modifier::BOLD));
            }
            Tag::Strikethrough => {
                self.style_stack
                    .push(Style::default().add_modifier(Modifier::CROSSED_OUT));
            }
            Tag::Link { dest_url, .. } => {
                self.style_stack.push(self.theme.link);
                self.pending_link_url = Some(dest_url.to_string());
            }
            Tag::Image { dest_url, .. } => {
                self.image_rendered = false;

                let resolved = self
                    .base_dir
                    .as_ref()
                    .map(|dir| dir.join(dest_url.as_ref()))
                    .filter(|p| p.is_file());

                if let Some(ref path) = resolved
                    && let Some(image_lines) = super::image::render_image(path)
                {
                    self.flush_line();
                    self.lines.extend(image_lines);
                    self.image_rendered = true;
                }

                if !self.image_rendered {
                    self.spans
                        .push(Span::styled("🖼 ", Style::default().fg(Color::DarkGray)));
                }

                self.style_stack.push(self.theme.link);
                self.pending_link_url = Some(dest_url.to_string());
            }
            _ => {}
        }
    }

    fn end_tag(&mut self, tag: TagEnd) {
        match tag {
            TagEnd::Paragraph => {
                if self.blockquote_depth > 0 {
                    self.flush_line();
                } else {
                    self.flush_line();
                    self.needs_newline = true;
                }
            }
            TagEnd::Heading(level) => {
                let heading_text: String = self.spans.iter().map(|s| s.content.as_ref()).collect();
                self.spans.clear();
                self.render_heading(level, &heading_text);
                self.heading = None;
                self.needs_newline = true;
            }
            TagEnd::BlockQuote(_) => {
                self.blockquote_depth = self.blockquote_depth.saturating_sub(1);
                if self.blockquote_depth == 0 {
                    self.needs_newline = true;
                }
            }
            TagEnd::CodeBlock => {
                if let Some(cb) = self.code_block.take() {
                    self.render_code_block(&cb);
                }
                self.needs_newline = true;
            }
            TagEnd::List(_) => {
                self.list_stack.pop();
                if self.list_stack.is_empty() {
                    self.needs_newline = true;
                }
            }
            TagEnd::Item => {
                self.flush_line();
            }
            TagEnd::Table => {
                if let Some(table) = self.table.take() {
                    self.render_table(&table);
                }
                self.needs_newline = true;
            }
            TagEnd::TableHead => {
                if let Some(table) = &mut self.table {
                    let cells = std::mem::take(&mut table.current_cells);
                    table.rows.push(TableRow {
                        cells,
                        is_header: true,
                    });
                    table.in_header = false;
                }
            }
            TagEnd::TableRow => {
                if let Some(table) = &mut self.table
                    && !table.in_header
                {
                    let cells = std::mem::take(&mut table.current_cells);
                    table.rows.push(TableRow {
                        cells,
                        is_header: false,
                    });
                }
            }
            TagEnd::TableCell => {
                if let Some(table) = &mut self.table {
                    let cell = std::mem::take(&mut table.current_cell);
                    table.current_cells.push(cell);
                }
            }
            TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough => {
                self.style_stack.pop();
            }
            TagEnd::Link => {
                self.style_stack.pop();
                if let Some(url) = self.pending_link_url.take() {
                    let link_text = std::mem::take(&mut self.pending_link_text);
                    self.links.push(LinkInfo {
                        url: url.clone(),
                        text: link_text,
                    });
                    self.spans.push(Span::styled(
                        format!(" ({})", url),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
            TagEnd::Image => {
                self.style_stack.pop();
                if self.image_rendered {
                    self.pending_link_url.take();
                    self.image_rendered = false;
                } else if let Some(url) = self.pending_link_url.take() {
                    self.spans.push(Span::styled(
                        format!(" ({})", url),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
            _ => {}
        }
    }

    fn text(&mut self, text: &str) {
        if let Some(ref mut cb) = self.code_block {
            cb.content.push_str(text);
            return;
        }

        if self.image_rendered {
            return;
        }

        if self.pending_link_url.is_some() {
            self.pending_link_text.push_str(text);
        }

        {
            let style = self.current_style();
            if let Some(table) = &mut self.table {
                table
                    .current_cell
                    .push(Span::styled(text.to_string(), style));
                return;
            }
        }

        if self.heading.is_some() {
            self.spans.push(Span::raw(text.to_string()));
            return;
        }

        if self.blockquote_depth > 0 && self.spans.is_empty() {
            let prefix = "│ ".repeat(self.blockquote_depth);
            self.spans.push(Span::styled(prefix, self.theme.blockquote));
        }

        let style = self.current_style();
        let parts: Vec<&str> = text.split('\n').collect();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                self.flush_line();
                if self.blockquote_depth > 0 {
                    let prefix = "│ ".repeat(self.blockquote_depth);
                    self.spans.push(Span::styled(prefix, self.theme.blockquote));
                }
            }
            if !part.is_empty() {
                self.spans.push(Span::styled(part.to_string(), style));
            }
        }
    }

    fn inline_code(&mut self, code: &str) {
        let styled = Span::styled(format!(" {} ", code), self.theme.code_inline);

        if let Some(ref mut table) = self.table {
            table.current_cell.push(styled);
            return;
        }

        self.spans.push(styled);
    }

    fn soft_break(&mut self) {
        if self.code_block.is_some() || self.table.is_some() {
            return;
        }
        self.spans.push(Span::raw(" "));
    }

    fn hard_break(&mut self) {
        self.flush_line();
    }

    fn rule(&mut self) {
        if self.needs_newline {
            self.push_blank();
        }
        self.lines.push(Line::styled("─".repeat(80), self.theme.hr));
        self.needs_newline = true;
    }

    fn task_list_marker(&mut self, checked: bool) {
        let marker = if checked { "☑ " } else { "☐ " };
        if let Some(last) = self.spans.last_mut() {
            *last = Span::styled(marker.to_string(), last.style);
        }
    }

    fn render_heading(&mut self, level: HeadingLevel, text: &str) {
        let style = match level {
            HeadingLevel::H1 => self.theme.h1,
            HeadingLevel::H2 => self.theme.h2,
            HeadingLevel::H3 => self.theme.h3,
            HeadingLevel::H4 => self.theme.h4,
            HeadingLevel::H5 => self.theme.h5,
            HeadingLevel::H6 => self.theme.h6,
        };

        self.headings.push(HeadingInfo {
            line: self.lines.len(),
            level,
            text: text.to_string(),
        });

        let padded = if level == HeadingLevel::H1 {
            format!("  {}  ", text)
        } else {
            text.to_string()
        };
        self.lines.push(Line::styled(padded, style));
    }

    /// Render a completed code block as a bordered box with syntax highlighting.
    /// Falls back to plain styled text if the language isn't recognized.
    fn render_code_block(&mut self, cb: &CodeBlockState) {
        let syn_theme = &THEME_SET.themes[&self.theme.syntect_theme];
        let bg_color = syn_theme
            .settings
            .background
            .map(|c| Color::Rgb(c.r, c.g, c.b))
            .unwrap_or(Color::Rgb(43, 48, 59));
        let bg = Style::default().bg(bg_color);
        let border = Style::default().fg(Color::Rgb(80, 80, 80)).bg(bg_color);
        let label_style = Style::default().fg(Color::Rgb(120, 120, 120)).bg(bg_color);
        let plain_code = Style::default().fg(Color::Rgb(192, 197, 206)).bg(bg_color);

        let syntax = cb
            .lang
            .as_ref()
            .and_then(|lang| SYNTAX_SET.find_syntax_by_token(lang));

        let content = cb.content.trim_end_matches('\n');
        let max_line_len = content.lines().map(|l| l.len()).max().unwrap_or(0);
        let box_width = max_line_len.max(20) + 4;

        let top = format!("  ╭{}╮", "─".repeat(box_width));
        self.lines.push(Line::styled(top, border));

        if let Some(lang) = &cb.lang {
            let pad = box_width.saturating_sub(lang.len() + 2);
            let mut line = Line::from(vec![
                Span::styled("  │ ", border),
                Span::styled(lang.to_string(), label_style),
                Span::styled(format!("{} │", " ".repeat(pad)), border),
            ]);
            line.style = bg;
            self.lines.push(line);
        }

        if content.is_empty() {
            let mut line = Line::from(vec![
                Span::styled("  │ ", border),
                Span::styled(" ".repeat(box_width - 2), bg),
                Span::styled(" │", border),
            ]);
            line.style = bg;
            self.lines.push(line);
        } else {
            let mut highlighter =
                syntax.map(|syn| HighlightLines::new(syn, syn_theme));

            for code_line in content.lines() {
                let mut line_spans = vec![Span::styled("  │ ", border)];

                if let Some(ref mut hl) = highlighter {
                    match hl.highlight_line(code_line, &SYNTAX_SET) {
                        Ok(ranges) => {
                            for (style, text) in ranges {
                                let fg = style.foreground;
                                line_spans.push(Span::styled(
                                    text.to_string(),
                                    Style::default()
                                        .fg(Color::Rgb(fg.r, fg.g, fg.b))
                                        .bg(bg_color),
                                ));
                            }
                        }
                        Err(_) => {
                            line_spans
                                .push(Span::styled(code_line.to_string(), plain_code));
                        }
                    }
                } else {
                    line_spans.push(Span::styled(code_line.to_string(), plain_code));
                }

                let rendered_len: usize = line_spans[1..]
                    .iter()
                    .map(|s| s.content.len())
                    .sum();
                let pad = box_width.saturating_sub(rendered_len + 2);
                line_spans.push(Span::styled(format!("{} │", " ".repeat(pad)), border));

                let mut line = Line::from(line_spans);
                line.style = bg;
                self.lines.push(line);
            }
        }

        let bottom = format!("  ╰{}╯", "─".repeat(box_width));
        self.lines.push(Line::styled(bottom, border));
    }

    /// Render a completed table with box-drawing borders. Column widths are computed
    /// using [`Span::width()`] (Unicode display width) to handle multi-byte characters correctly.
    fn render_table(&mut self, table: &TableState) {
        if table.rows.is_empty() {
            return;
        }

        let num_cols = table.rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);

        if num_cols == 0 {
            return;
        }

        let mut col_widths = vec![0usize; num_cols];
        for row in &table.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i < num_cols {
                    let width: usize = cell.iter().map(|s| s.width()).sum();
                    col_widths[i] = col_widths[i].max(width);
                }
            }
        }
        for w in &mut col_widths {
            *w = (*w).max(3);
        }

        let bs = self.theme.table_border;

        self.lines.push(Line::styled(
            Self::table_border(&col_widths, '┌', '┬', '┐'),
            bs,
        ));

        for row in &table.rows {
            let mut spans = Vec::new();
            spans.push(Span::styled("│", bs));

            for (i, w) in col_widths.iter().enumerate() {
                let cell_spans = row.cells.get(i);
                let cell_width: usize = cell_spans
                    .map(|c| c.iter().map(|s| s.width()).sum())
                    .unwrap_or(0);
                let cell_text: String = cell_spans
                    .map(|c| c.iter().map(|s| s.content.as_ref()).collect())
                    .unwrap_or_default();

                let padding = w.saturating_sub(cell_width);
                let padded = format!(" {}{} ", cell_text, " ".repeat(padding));

                let cell_style = if row.is_header {
                    self.theme.table_header
                } else {
                    Style::default()
                };

                spans.push(Span::styled(padded, cell_style));
                spans.push(Span::styled("│", bs));
            }

            self.lines.push(Line::from(spans));

            if row.is_header {
                self.lines.push(Line::styled(
                    Self::table_border(&col_widths, '├', '┼', '┤'),
                    bs,
                ));
            }
        }

        self.lines.push(Line::styled(
            Self::table_border(&col_widths, '└', '┴', '┘'),
            bs,
        ));
    }

    fn table_border(col_widths: &[usize], left: char, mid: char, right: char) -> String {
        let mut s = String::new();
        s.push(left);
        for (i, w) in col_widths.iter().enumerate() {
            for _ in 0..(w + 2) {
                s.push('─');
            }
            if i < col_widths.len() - 1 {
                s.push(mid);
            } else {
                s.push(right);
            }
        }
        s
    }

    fn into_output(mut self) -> RenderOutput {
        self.flush_line();
        RenderOutput {
            text: Text::from(self.lines),
            headings: self.headings,
            links: self.links,
        }
    }
}
