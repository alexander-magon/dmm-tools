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

## Adding a New Device Model (same protocol family)

To add a new meter that shares an existing protocol (e.g. another UT61x variant):

1. Create `crates/ut61eplus-lib/src/protocol/ut61eplus/tables/new_model.rs`
2. Implement the `DeviceTable` trait with mode/range tables for the new model
3. Register it in `protocol/ut61eplus/tables/mod.rs`

## Adding a New Protocol Family

To add support for a device family with a different wire protocol:

1. Create `crates/ut61eplus-lib/src/protocol/newfamily/mod.rs`
2. Implement the `Protocol` trait (`init`, `request_measurement`, `send_command`, `get_name`, `profile`, `capture_steps`)
3. Add a variant to the `DeviceFamily` enum in `protocol/mod.rs`
4. Add `Display`, `FromStr`, and `activation_instructions()` for the new variant
5. Add the match arm in `open_device()` in `lib.rs`
6. Add the `--device` alias in the CLI's `parse_device_family()` (if needed)
7. Add the device to the GUI selector in `app.rs` (`device_display_name` and settings panel)
8. Create research docs in `docs/research/newfamily/` documenting the wire protocol
9. Set `Stability::Experimental` in the `DeviceProfile` until verified against real hardware

The `Protocol` trait is object-safe and `Send`, so the new family works automatically
with `Dmm<T>`, the CLI, and (eventually) the GUI.

## Golden File Tests

Golden file tests verify measurement parsing against known-good byte sequences.
Each `.json` file in `crates/ut61eplus-lib/tests/golden/<family>/` contains a
hex-encoded payload and the expected parsed fields (mode, value, unit, range, flags).

To add a golden test:

1. Capture a raw payload from a real device (via `ut61eplus debug` with `RUST_LOG=trace`)
2. Create a `.json` file in the appropriate `tests/golden/<family>/` directory
3. Fill in `payload_hex` and the expected parsed fields
4. Run `cargo test --workspace` to verify

Golden tests run as part of the standard test suite. They are the primary
regression safety net for protocol parsing — add them whenever you verify
a new mode/range/flag combination against real hardware.

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
