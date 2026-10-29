# Rust / Cargo

Apply only to Cargo projects. Follow rust-toolchain.toml, the declared minimum supported Rust version, workspace structure, targets, and feature policy. Use stable tooling unless the project explicitly requires nightly. Preserve Cargo.lock according to project policy; use `--locked` where a lockfile is committed.

Prefer clear ownership and error types, avoid unnecessary clones and dependencies, and document unsafe code's invariants. Treat CLI arguments, files, and network input as untrusted. Test CLI stdout/stderr, exit codes, malformed input, and filesystem effects using temporary directories.

Run applicable `cargo test --locked`, `cargo fmt --check`, `cargo clippy --locked -- -D warnings`, and `cargo build --locked`. Adapt commands to the actual workspace, targets, and supported feature combinations. Do not enable all features blindly when features are mutually exclusive. Report unavailable cross-compilation targets and external services. Do not run cargo publish without explicit authorization.
