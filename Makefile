WEECHAT_HOME ?= $(HOME)/.weechat
PREFIX ?= $(WEECHAT_HOME)
SOURCES = src/*.rs

.PHONY: format install install-dir

target/debug/libbuffer_autohide.so: $(SOURCES)
	cargo build

install: target/debug/libbuffer_autohide.so install-dir
	install -m644 target/debug/libbuffer_autohide.so $(DESTDIR)$(PREFIX)/plugins/libbuffer_autohide.so

install-dir:
	install -d $(DESTDIR)$(PREFIX)/plugins

format:
	cargo fmt
