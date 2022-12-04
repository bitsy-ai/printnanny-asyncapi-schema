.PHONY: clients-release

CLIENT_DIR ?= $(PWD)/clients/

RUST_CLIENT_DIR ?= $(CLIENT_DIR)/rust
TYPESCRIPT_CLIENT_DIR ?= $(CLIENT_DIR)/typescript

rust-clean:
	rm -rf $(RUST_CLIENT_DIR)

rust-client-build: rust-client-clean
	cd $(CLIENT_DIR) && npm run generate:rust

rust-client-release: rust-client-build
	cd $(RUST_CLIENT_DIR) && cargo publish

typescript-clean:
	rm -rf $(CLIENT_DIR)/typescript

typescript-client-build: typescript-clean
	cd $(CLIENT_DIR) && npm run generate:typescript

typescript-client-release: typescript-client-build
	cd $(CLIENT_DIR) && npm publish

clients-release: rust-client-release typescript-client-release
