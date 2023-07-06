all: run

run:
	scripts/record.sh
	cargo run

.PHONY: all run
