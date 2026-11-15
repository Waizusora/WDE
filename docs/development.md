# Development

## Toolchain

Use the stable Rust toolchain. The repository's `rust-toolchain.toml` requests the `rustfmt` and `clippy` components.

## Local checks

Run these commands before submitting changes:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Crate boundaries

- Keep each crate focused on one responsibility.
- Avoid circular dependencies.
- Put shared data types in `wde-common` only when they are genuinely shared.
- Put stable cross-process interfaces in `wde-protocols`.
- Do not expose Smithay types through public WDE APIs unless there is a documented reason.
- Do not add a dependency before its full license tree has been reviewed.

## Change policy

Changes to public interfaces, runtime architecture, dependency policy, or default visual behavior must be documented in the relevant file under `docs/`.
