.PHONY: update_toml test_crypt test_io test_qr test_webscrape test_all

define runtest
	cargo test --features $(1)
endef

test_crypt: #Test crypt features flag
	$(call runtest,crypt)

test_io: #Test io features flag
	$(call runtest,io)

test_qr: #Test qr features flag
	$(call runtest,qr)

test_webscrape: #Test webscrape features flag
	$(call runtest,webscrape)

test_all: #Run test for all features
	cargo test --workspace --all-features

update_toml: #Update dependencies in Cargo.toml
	cargo update --manifest-path Cargo.toml