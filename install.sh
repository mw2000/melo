#!/usr/bin/env sh
set -eu

REPO="mw2000/mdfi"
INSTALL_DIR="${MDFI_INSTALL_DIR:-$HOME/.local/bin}"

main() {
    platform="$(detect_platform)"
    if [ -z "$platform" ]; then
        err "unsupported platform: $(uname -s) $(uname -m)"
    fi

    tag="$(get_latest_tag)"
    if [ -z "$tag" ]; then
        err "could not determine latest release"
    fi

    url="https://github.com/${REPO}/releases/download/${tag}/mdfi-${platform}.tar.gz"

    info "downloading mdfi ${tag} for ${platform}"

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$tmpdir/mdfi.tar.gz"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$tmpdir/mdfi.tar.gz" "$url"
    else
        err "curl or wget required"
    fi

    tar xzf "$tmpdir/mdfi.tar.gz" -C "$tmpdir"

    mkdir -p "$INSTALL_DIR"
    mv "$tmpdir/mdfi" "$INSTALL_DIR/mdfi"
    chmod +x "$INSTALL_DIR/mdfi"

    info "installed to ${INSTALL_DIR}/mdfi"

    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
        warn "${INSTALL_DIR} is not in your PATH"
        warn "add this to your ~/.zshrc or ~/.bashrc:"
        warn ""
        warn "  export PATH=\"${INSTALL_DIR}:\$PATH\""
    fi

    info "done — run 'mdfi --help' to get started"
}

detect_platform() {
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Darwin)
            case "$arch" in
                arm64|aarch64) echo "aarch64-apple-darwin" ;;
                x86_64)        echo "x86_64-apple-darwin" ;;
            esac
            ;;
        Linux)
            case "$arch" in
                x86_64)        echo "x86_64-unknown-linux-gnu" ;;
                aarch64)       echo "aarch64-unknown-linux-gnu" ;;
            esac
            ;;
    esac
}

get_latest_tag() {
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null \
            | grep '"tag_name"' | head -1 | cut -d'"' -f4
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" 2>/dev/null \
            | grep '"tag_name"' | head -1 | cut -d'"' -f4
    fi
}

info() { printf '\033[1;32m%s\033[0m\n' "$*"; }
warn() { printf '\033[1;33m%s\033[0m\n' "$*"; }
err()  { printf '\033[1;31merror: %s\033[0m\n' "$*" >&2; exit 1; }

main
