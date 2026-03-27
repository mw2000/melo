# mdfi

A terminal markdown viewer. Read `.md` files comfortably in your terminal.

## Installation

```sh
curl -fsSL https://raw.githubusercontent.com/mw2000/mdfi/main/install.sh | sh
```

This downloads the latest release binary to `~/.local/bin`. Make sure it's on your PATH:

```sh
export PATH="$HOME/.local/bin:$PATH"
```

### From source

```sh
just install
```

Or with cargo directly: `cargo install --path .` (requires Rust 1.85+).

## Usage

```sh
mdfi <file>
```

Pipe from stdin:

```sh
cat README.md | mdfi
curl -s https://example.com/doc.md | mdfi
```

Pass `--help` or `--version` for the usual.

## Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Scroll down |
| `k` / `↑` | Scroll up |
| `Ctrl-d` / `PageDown` | Half-page down |
| `Ctrl-u` / `PageUp` | Half-page up |
| `g` / `Home` | Jump to top |
| `G` / `End` | Jump to bottom |
| `/` | Search |
| `n` / `N` | Next / previous match |
| `?` | Toggle help overlay |
| `q` / `Esc` | Quit |

Mouse scroll is supported.

## Built With

- [ratatui](https://github.com/ratatui-org/ratatui) 0.30
- [crossterm](https://github.com/crossterm-rs/crossterm) 0.29
- [tui-markdown](https://github.com/joshka/tui-markdown) 0.3

## License

MIT
