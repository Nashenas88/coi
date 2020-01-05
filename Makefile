.PHONY: test check clippy

# Find all directories in the current dir that have Cargo.toml files
CRATES=$(dir $(wildcard */Cargo.toml))

test:
	for dir in $(CRATES); do\
		echo "$$dir"; \
		cd "$$dir"; \
		cargo test; \
		cd ..; \
	done

check:
	for dir in $(CRATES); do\
		echo "$$dir"; \
		cd "$$dir"; \
		cargo check; \
		cd ..; \
	done

clippy:
	for dir in $(CRATES); do\
		echo "$$dir"; \
		cd "$$dir"; \
		cargo clippy; \
		cd ..; \
	done

