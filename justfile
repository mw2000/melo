default_prefix := env("HOME") + "/.local"

build:
    cargo build --release

install prefix=default_prefix: build
    mkdir -p {{prefix}}/bin
    cp target/release/mdfi {{prefix}}/bin/mdfi

uninstall prefix=default_prefix:
    rm -f {{prefix}}/bin/mdfi
