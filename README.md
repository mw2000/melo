# 🌿 melo

A mellow terminal markdown viewer. Read `.md` files comfortably in your terminal.

## Installation

```sh
curl -fsSL https://raw.githubusercontent.com/mw2000/melo/main/install.sh | sh
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
melo <file>
```

Pipe from stdin:

```sh
cat README.md | melo
curl -s https://example.com/doc.md | melo
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
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) 0.13
- [syntect](https://github.com/trishume/syntect) 5

## License

MIT
