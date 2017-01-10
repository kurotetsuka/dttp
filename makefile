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
test: test-protocol
test-all:
	cargo test

test-client:
	cargo run --bin test_client
test-daemon:
	cargo run --bin test_daemon
test-gpg:
	cargo test --test gpg
test-keybase:
	cargo run --bin test_keybase
test-mageon:
	cargo run --bin test_mageon
test-protocol:
	cargo run --bin test_protocol
