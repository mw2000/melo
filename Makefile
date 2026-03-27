PREFIX ?= $(HOME)/.local

build:
	cargo build --release

install: build
	mkdir -p $(PREFIX)/bin
	cp target/release/mdfi $(PREFIX)/bin/mdfi

uninstall:
	rm -f $(PREFIX)/bin/mdfi

.PHONY: build install uninstall
