.PHONY: clients-release

CLIENT_DIR ?= $(PWD)

RUST_CLIENT_DIR ?= $(CLIENT_DIR)/rust
TYPESCRIPT_CLIENT_DIR ?= $(CLIENT_DIR)/typescript

$(TYPESCRIPT_CLIENT_DIR):
	mkdir -p $(TYPESCRIPT_CLIENT_DIR)

$(RUST_CLIENT_DIR):
	mkdir -p $(RUST_CLIENT_DIR)

rust-clean:
	rm -rf $(RUST_CLIENT_DIR)

rust-client-build: rust-client-clean $(RUST_CLIENT_DIR)
	cd $(CLIENT_DIR) && npm run generate:rust

rust-client-release: rust-client-build
	cd $(RUST_CLIENT_DIR) && cargo publish

rust-client-clean:
	rm -rf $(RUST_CLIENT_DIR)

typescript-clean:
	rm -rf $(TYPESCRIPT_CLIENT_DIR)

typescript-client-build: typescript-clean $(TYPESCRIPT_CLIENT_DIR)
	cd $(CLIENT_DIR) && npm run generate:typescript

typescript-client-release: typescript-client-build
	cd $(CLIENT_DIR) && npm publish


clients-build: typescript-client-build rust-client-build

clients-release: rust-client-release typescript-client-release
