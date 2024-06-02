all: build

build:
	scripts/record.sh
	cargo build

run:
	scripts/record.sh
	cargo run

.PHONY: all build run
