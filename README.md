# PrintNanny.ai AsyncAPI Schemas

## Generate + build a language client

```bash
cd clients/<package directory>/
make <language>-client-build
```

Supported languages: Rust, Python, Typescript

## Release a new package version

```bash
cd clients/<package directory>/
bump2version --new-version=<version>
make clients-release
```