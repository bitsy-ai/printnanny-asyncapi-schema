.PHONY: clients-release

CLIENT_DIR ?= $(PWD)/clients/

RUST_CLIENT_DIR ?= $(CLIENT_DIR)/rust
TYPESCRIPT_CLIENT_DIR ?= $(CLIENT_DIR)/typescript

rust-clean:
	rm -rf $(RUST_CLIENT_DIR)

rust-client-build:
	cd $(CLIENT_DIR) && npm run generate:rust

rust-client-release: rust-client-clean rust-client-build
	cd $(RUST_CLIENT_DIR) && cargo publish

typescript-clean:
	rm -rf $(CLIENT_DIR)/typescript

typescript-client-build:
	cd $(CLIENT_DIR) && npm run generate:typescript

typescript-client-release:
	cd $(CLIENT_DIR) && npm run publish

clients-release: rust-client-release typescript-client-release