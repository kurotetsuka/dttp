# globals
default: build
freshen: clean build
clean:
	cargo clean

# vars

# commands
build:
	cargo build

update:
	cargo update

ci:
	make-ci build $$(find src -name *.rs)

# tests
test: test-keybase
test-all:
	cargo test

test-keybase:
	cargo test --test test_keybase

test-gpg:
	cargo test --test gpg
