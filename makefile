# globals
default: build
freshen: clean build
clean:
	cargo clean

# vars

# commands
build:
	cargo build

ci:
	make-ci build $$(find src -name *.rs)

# tests
test:
	cargo test
