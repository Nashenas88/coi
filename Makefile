.PHONY: test check clippy fmt

# Find all directories in the current dir that have Cargo.toml files
CRATES=$(dir $(wildcard **/Cargo.toml))

define run_on_crates
	failed=false; \
	for dir in $(CRATES); do \
		echo "$$dir"; \
		cd "$$dir"; \
		cargo $(1) $(EXTRA) || failed=true; \
		cd ..; \
	done; \
	if [ "$$failed" = "true" ]; then exit 1; fi
endef

test:
	$(call run_on_crates, test)

check:
	$(call run_on_crates, check)

clippy:
	$(call run_on_crates, clippy)

fmt:
	$(call run_on_crates, fmt)
