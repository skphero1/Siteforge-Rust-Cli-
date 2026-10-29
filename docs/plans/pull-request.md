# Build SiteForge v0.1.0 technical SEO auditing CLI

SiteForge turns this starter repository into a Rust CLI for bounded same-origin
website audits. It reports HTTP failures, redirect chains, SEO markup findings,
public WordPress indicators, robots policy and sitemap status in terminal,
JSON, CSV or standalone HTML output.

Requests use validated, pinned DNS answers, reject non-public destinations by
default, enforce robots exclusions and crawl-delay, bound response bodies and
crawl work, and stop the origin on authentication denial or rate limiting.
`--allow-private` explicitly authorizes local/internal fixtures or audits.

Includes modular source, MIT licensing, usage documentation, a committed
Cargo.lock, fixture-server tests and Linux/Windows/macOS GitHub Actions.

Validation on Linux with stable Rust 1.99.1:
- `cargo fmt --check` passed.
- `cargo clippy --locked --all-targets -- -D warnings` passed.
- `cargo test --locked` passed: 18 tests (7 unit, 11 integration); no ignored tests.
- `cargo build --locked --release` passed.
- `python3 scripts/verify-template.py` passed.
- `python3 -m unittest discover -s tests -v` passed: 8 tests.
- `git diff --check` passed.

Windows/macOS execution awaits CI; this is a self-review, not independent
approval. Sitemap inspection is shallow; external links/assets, JavaScript
rendering and authenticated pages are outside the implemented crawl scope.
Initial toolchain absence prevented a complete red/green TDD sequence. Rust
fixture tests require local socket permissions in a restricted sandbox.
