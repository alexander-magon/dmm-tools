# Development

## Running Tests

```sh
cargo test --workspace
```

All tests use `MockTransport` and run without hardware connected.

## Linting

```sh
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

## Adding a New Device Model

1. Create `crates/ut61eplus-lib/src/tables/new_model.rs`
2. Implement the `DeviceTable` trait with mode/range tables for the new model
3. Register it in `tables/mod.rs`

## Release Process

*To be defined.*
