# Makefile - teensy_strawman_rust
# Build and flash the bare-metal Rust app to a Teensy 4.1 (i.MX RT1062).
#
# Builds via cargo; flashes via teensy_loader_cli. ELF -> Intel HEX uses the
# Rust-native rust-objcopy (cargo-binutils + llvm-tools) when present, else a GNU
# arm-eabi-objcopy. Override with OBJCOPY=... / SIZE=... if needed.
#
# A Cargo workspace with one crate per app (each has its own src/main.rs):
#   - blink   (blink/src/main.rs)   -- the simple loop blink  [default]
#   - tasking (tasking/src/main.rs) -- the RTIC async tasking blink
#
# Common usage:
#   make                # build the default (blink) binary
#   make burn           # build + hex + flash the blink (press the onboard button)
#   make burn-blink     # convenience: build + flash the simple blink
#   make burn-tasking   # convenience: build + flash the RTIC tasking blink
#   make burn BIN=tasking   # explicit form of burn-tasking
#   make size / make clean / make help

TARGET  := thumbv7em-none-eabihf
PROFILE := release
# Default binary is the simple blink; override with BIN=tasking (or use burn-tasking)
BIN     ?= blink
ELF     := target/$(TARGET)/$(PROFILE)/$(BIN)
HEX     := bin/$(BIN).hex
MCU     := TEENSY41

# objcopy/size: prefer the Rust-native rust-objcopy/rust-size (cargo-binutils),
# else a GNU arm-eabi tool on PATH, else one from the Alire GNAT toolchain.
OBJCOPY ?= $(shell command -v rust-objcopy 2>/dev/null || command -v arm-eabi-objcopy 2>/dev/null || command -v arm-none-eabi-objcopy 2>/dev/null || find $(HOME)/.local/share/alire/toolchains -name 'arm-eabi-objcopy' 2>/dev/null | head -1)
SIZE    ?= $(shell command -v rust-size 2>/dev/null || command -v arm-eabi-size 2>/dev/null || command -v arm-none-eabi-size 2>/dev/null || find $(HOME)/.local/share/alire/toolchains -name 'arm-eabi-size' 2>/dev/null | head -1)

.DEFAULT_GOAL := build
.PHONY: build hex burn burn-blink burn-tasking flash size clean help

build: ## Build the ELF (cargo build --release -p $(BIN))
	cargo build --$(PROFILE) -p $(BIN)

hex: build ## Build and generate the Intel HEX image
	@mkdir -p bin
	$(OBJCOPY) -O ihex $(ELF) $(HEX)
	@echo "HEX image ready: $(HEX)"

burn: hex ## Build, make HEX, and flash to the board (press the onboard button)
	@echo ">>> Flashing $(MCU) [$(BIN)]: press the Teensy onboard button when prompted..."
	teensy_loader_cli --mcu=$(MCU) -w -v $(HEX)

flash: burn ## Alias for 'burn'

burn-blink: ## Convenience: build + flash the simple loop blink
	$(MAKE) burn BIN=blink

burn-tasking: ## Convenience: build + flash the RTIC tasking blink
	$(MAKE) burn BIN=tasking

size: build ## Report image section sizes
	$(SIZE) $(ELF)

clean: ## Remove build artifacts (target/, bin/)
	cargo clean
	rm -rf bin

help: ## List available targets
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
	  | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-13s\033[0m %s\n", $$1, $$2}'
