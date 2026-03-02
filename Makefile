.PHONY: all release install clean help

# Directories
BIN_DIR = $(HOME)/bin
TARGET_DIR = target/release

# Binaries
BINS = jki jkim jki-agent

all: help

## release: Build release binaries
release:
	cargo build --release --workspace

## install: Build and deploy binaries to ~/bin/
install: release
	@mkdir -p $(BIN_DIR)
	@for bin in $(BINS); do \
		echo "Installing $$bin to $(BIN_DIR)..."; \
		cp $(TARGET_DIR)/$$bin $(BIN_DIR)/; \
	done
	@echo ""
	@echo "Installation complete! Binaries are in $(BIN_DIR)."
	@echo "Ensure $(BIN_DIR) is in your PATH."

## clean: Remove build artifacts
clean:
	cargo clean

## help: Show this help message
help:
	@echo "Just Keep Identity (jki) Build & Deploy Tool"
	@echo ""
	@echo "Usage:"
	@echo "  make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^##' Makefile | sed -e 's/## //g' | column -t -s ':'
