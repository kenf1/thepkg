.PHONY: help update_toml fmt \
	test_crypt test_io test_qr test_webscrape test_all

help:
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} \
	/^[a-zA-Z0-9_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } \
	/^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) }' $(MAKEFILE_LIST)

test_crypt: ##Test crypt features flag
	cargo test --features "crypt"

test_io: ##Test io features flag
	cargo test --features "io"

test_qr: ##Test qr features flag
	cargo test --features "qr"

test_webscrape: ##Test webscrape features flag
	cargo test --features "webscrape"

test_all: ##Run test for all features
	cargo test --workspace --all-features

update_toml: ##Update dependencies in Cargo.toml
	cargo update --manifest-path Cargo.toml

fmt: ##Format
	cargo fmt