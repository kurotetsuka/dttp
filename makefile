# globals
default: build
freshen: clean build
clean:
	cargo clean

# vars

# commands
build:
	cargo build

# tests
test:
	cargo test
