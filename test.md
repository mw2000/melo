# Welcome to mdfi

A **TUI markdown viewer** built with Rust, ratatui, and crossterm.

## Features

- Smooth scrolling with vim-style keybindings
- Syntax highlighted code blocks
- *Italic*, **bold**, and ~~strikethrough~~ text
- Configurable themes via builder pattern

## Code Example

```rust
fn main() {
    let app = App::builder()
        .file("README.md".into())
        .theme(Theme::default())
        .build()
        .expect("failed to build app");

    app.run().unwrap();
}
```

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `d` / `PageDown` | Half-page down |
| `u` / `PageUp` | Half-page up |
| `g` | Jump to top |
| `G` | Jump to bottom |
| `q` / `Esc` | Quit |

## Architecture

The project follows a clean, modular architecture:

> Each module has a single responsibility and communicates
> through well-defined interfaces. Builder patterns are used
> for configuration, making the API ergonomic and extensible.

### Module Breakdown

1. **action.rs** — Pure enum of all possible actions
2. **input.rs** — Configurable key-to-action mapping
3. **event.rs** — Crossterm event polling wrapper
4. **terminal.rs** — RAII terminal guard
5. **markdown/** — Parsing and theming
6. **ui/** — Viewport and rendering
7. **app.rs** — Application state and lifecycle

---

### Links

Check out the [ratatui docs](https://docs.rs/ratatui) for more information.

---

*Built with love and Rust.*
