.PHONY: test
test:
	cargo test
	e2e/test.sh
