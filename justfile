default_prefix := env("HOME") + "/.local"

build:
    cargo build --release

install prefix=default_prefix: build
    mkdir -p {{prefix}}/bin
    cp target/release/melo {{prefix}}/bin/melo

uninstall prefix=default_prefix:
    rm -f {{prefix}}/bin/melo
