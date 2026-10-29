# SiteForge v0.1.0

Implement a portable Rust CLI with clap, Tokio/reqwest, scraper, serde, tracing,
and anyhow. Modules: network safety and bounded fetching; robots policy; HTML
analysis; same-origin bounded crawl; terminal/JSON/CSV/HTML reports.

Safety: HTTP(S) only, no URL credentials, reject non-public DNS answers, pin
validated addresses, disable implicit proxies and automatic redirects. Every
redirect gets the same validation. Explicit --allow-private authorizes local
fixtures. Fail closed on unavailable robots policy except 404/410. Stop an
origin on access denial or rate limiting; do not retry around restrictions.

Sequence: add behavior tests and fixtures, implement modules and CLI, add
cross-platform CI and usage/license docs, inspect security boundaries, run
cargo fmt --check, cargo clippy --all-targets -- -D warnings, cargo test,
cargo build, and existing Python template checks. Generate and commit Cargo.lock
when Cargo is available; use --locked thereafter. No services or secrets needed.

Acceptance: mock-server tests prove crawling, robots exclusions, redirects,
errors, limits, all report formats, invalid inputs and private-address blocking.
No live third-party site is needed for validation. Preserve existing checkout.

Initial environment limitation: Rust/Cargo were absent and the sandbox could not
reach the configured proxy. Supported sandbox escalation enabled installing
stable Rust and fetching locked dependencies with verification intact. The
initial test-first attempt could not execute before implementation; subsequent
Rust tests execute against local fixtures. Do not claim a complete red/green
implementation sequence.
