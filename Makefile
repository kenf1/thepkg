test_crypt:
	cargo test --features "crypt"

test_io:
	cargo test --features "io"

test_qr:
	cargo test --features "qr"

test_webscrape:
	cargo test --features "webscrape"

# all: test_crypt test_io test_qr test_webscrape