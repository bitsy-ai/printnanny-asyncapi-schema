.PHONY: clients-release

CLIENT_DIR ?= $(PWD)

RUST_CLIENT_DIR ?= $(CLIENT_DIR)/rust
TYPESCRIPT_CLIENT_DIR ?= $(CLIENT_DIR)/typescript
PYTHON_CLIENT_DIR ?= $(CLIENT_DIR)/python/src

$(TYPESCRIPT_CLIENT_DIR):
	mkdir -p $(TYPESCRIPT_CLIENT_DIR)

$(RUST_CLIENT_DIR):
	mkdir -p $(RUST_CLIENT_DIR)

$(PYTHON_CLIENT_DIR):
	mkdir -p $()


python-clean:
	rm -rf $(PYTHON_CLIENT_DIR)

python-client-build: python-clean
	npm run generate:python
	cd $(PYTHON_CLIENT_DIR) && cd .. && python3 -m build

python-client-release: python-client-build
	twine check $(CLIENT_DIR)/python/dist/*
	twine upload $(CLIENT_DIR)/python/dist/*

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


clients-build: typescript-client-build python-client-build rust-client-build 

clients-release: rust-client-release typescript-client-release python-client-release
