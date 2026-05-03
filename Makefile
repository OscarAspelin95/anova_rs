.PHONY: all fmt fix lint test check docs update clean ci strict-lint

all: update fmt fix lint test build-release

fmt:
	@cargo fmt --all

fix:
	@cargo fix --allow-dirty
	@cargo clippy --fix --all --allow-dirty

lint:
	@cargo clippy --all-features

strict-lint:
	@cargo clippy --all-features -- -D warnings

test:
	@cargo test

build-release:
	@cargo build --release

check:
	@cargo check --all-features

docs:
	@cargo doc -p anova_rs --open

update:
	@cargo update

clean:
	@cargo clean
