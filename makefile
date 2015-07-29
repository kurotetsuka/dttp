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
test: test-all
test-all:
	cargo test

test-sign:
	cargo test --test sign

test-verify:
	cargo test --test verify

test-key-import:
	cargo test --test key_import
