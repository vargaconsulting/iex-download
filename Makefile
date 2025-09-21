# Makefile

all: build

build:
	cargo build --release

run:
	cargo run --release

test:
	cargo test

clean:
	cargo clean
