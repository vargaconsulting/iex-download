# ALL RIGHTS RESERVED.
# ___________________________________________________________________________________
# NOTICE: All information contained herein is, and remains the property of Varga LABS
# and its suppliers, if any. The intellectual and technical concepts contained herein 
# are  proprietary to Varga LABS and its suppliers and may be covered by Canadian and 
# Foreign Patents, patents in process, and are protected by trade secret or copyright 
# law. Dissemination of this information or reproduction of this material is strictly
# forbidden unless prior written permission is obtained from Varga LABS.
#
# Copyright © 2017-2025 Varga LABS, Toronto, On                    info@vargalabs.com
# ___________________________________________________________________________________

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

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/$(NAME)
