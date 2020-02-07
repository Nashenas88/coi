.PHONY: test check clippy

# Find all directories in the current dir that have Cargo.toml files
PRE_CRATES=$(dir $(wildcard **/Cargo.toml))
CRATES=$(PRE_CRATES) coi/coi-derive/

test:
	failed=false; \
	for dir in $(CRATES); do \
		echo "$$dir"; \
		cd "$$dir"; \
		cargo test $(EXTRA) || failed=true; \
		cd ..; \
	done; \
	if [ "$$failed" = "true" ]; then exit 1; fi

check:
	failed=false; \
	for dir in $(CRATES); do\
		echo "$$dir"; \
		cd "$$dir"; \
		cargo check $(EXTRA) || failed=true; \
		cd ..; \
	done; \
	if [ "$$failed" = "true" ]; then exit 1; fi

clippy:
	failed=false; \
	for dir in $(CRATES); do\
		echo "$$dir"; \
		cd "$$dir"; \
		cargo clippy $(EXTRA) || failed=true; \
		cd ..; \
	done; \
	if [ "$$failed" = "true" ]; then exit 1; fi
