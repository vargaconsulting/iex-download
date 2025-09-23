# This file is part of the IEX2H5 project and is licensed under the MIT License.
# 
# Copyright © 2017–2025 Varga LABS, Toronto, ON, Canada 🇨🇦
# Contact: info@vargalabs.com 


PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
NAME   := iex-download

all: build

build:
	cargo build --release
	strip target/release/$(NAME)

run:
	cargo run --release

test:
	cargo test

clean:
	cargo clean

install: build
	sudo install -Dm755 target/release/$(NAME) $(DESTDIR)$(BINDIR)/$(NAME)
	sudo cp iex-download.1 /usr/local/share/man/man1/
uninstall:
	rm -f $(DESTDIR)$(BINDIR)/$(NAME)
