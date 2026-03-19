# Development

## Setup

After cloning, install the pre-commit hooks:

```sh
ln -sf ../../git-hooks/pre-commit .git/hooks/pre-commit
```

This runs `cargo fmt --check`, `cargo clippy`, and `cargo test` before each commit.

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

1. Set the release version in root `Cargo.toml` (workspace inherits it), e.g. `version = "0.2.0"`
2. Commit: `git commit -am "Release v0.2.0"`
3. Tag: `git tag v0.2.0 && git push && git push origin v0.2.0`
4. The `release.yml` GitHub Actions workflow builds Linux and Windows binaries and creates a GitHub Release automatically
5. Bump to the next dev version: set `version = "0.3.0-dev"` in `Cargo.toml`, commit, and push

## Shell Completions

Generate completions for your shell:

```sh
ut61eplus completions bash > ~/.local/share/bash-completion/completions/ut61eplus
ut61eplus completions zsh > ~/.zfunc/_ut61eplus
ut61eplus completions fish > ~/.config/fish/completions/ut61eplus.fish
ut61eplus completions powershell >> $PROFILE
```
