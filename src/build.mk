SHELL ?= /bin/bash

CURRENT_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
SERVER_DIR := $(CURRENT_DIR)/outpost_server
CLIENT_DIR := $(CURRENT_DIR)/outpost_client
BUILD_OUTPUT_DIR := $(CURRENT_DIR)/../build

prepare_output_directory:
	@rm -rf $(BUILD_OUTPUT_DIR)
	@mkdir -p $(BUILD_OUTPUT_DIR)

build: prepare_output_directory
	cargo build --release --manifest-path $(SERVER_DIR)/Cargo.toml
	cp $(SERVER_DIR)/target/release/outpost_server $(BUILD_OUTPUT_DIR)

	cargo build --release --manifest-path $(CLIENT_DIR)/Cargo.toml
	cp $(CLIENT_DIR)/target/release/outpost_client $(BUILD_OUTPUT_DIR)