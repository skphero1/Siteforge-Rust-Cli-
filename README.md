# SiteForge

A free, open-source technical SEO auditing CLI in Rust. No accounts, AI APIs,
paid services, browser, or database required. Version 0.1.0.

## Install

Install current stable Rust using the official rustup installer, then:

```sh
git clone https://github.com/skphero1/Siteforge-Rust-Cli-.git
cd Siteforge-Rust-Cli-
cargo install --locked --path .
siteforge --help
```

Supports Windows, Linux and macOS. Rust 1.85+ syntax; current stable is the
supported toolchain. Cargo.lock is committed for reproducible dependency resolution.

## Usage

```sh
siteforge audit https://example.com
siteforge audit https://example.com --max-pages 100 --concurrency 4
siteforge audit https://example.com --format json > audit.json
siteforge audit https://example.com --format csv > audit.csv
siteforge audit https://example.com --format html > audit.html
siteforge audit https://example.com --delay-ms 1000 --timeout-secs 20
```

Reports go to stdout; errors and optional `RUST_LOG=info` diagnostics go to stderr.
Exit 0 means the audit completed, including audits with SEO findings or HTTP
errors. Invalid arguments, unavailable robots policy, and initial network safety
failures exit nonzero. Page-level failures remain in the report. Inspect
`stopped`, per-page errors and status codes when consuming reports.

The default is 100 pages, 4 concurrent requests, 500 ms between request starts,
15 seconds per HTTP request and a 2 MiB body limit. The body cap applies to
robots and sitemap responses too. Redirects are recorded and followed manually,
up to 10 hops, within the original scheme/host/port. Crawl limits count page
jobs; redirects and the two discovery requests are additional HTTP requests.
The discovery frontier is bounded to 20 times the page limit. Query strings are
retained and fragments removed. Findings cover missing or duplicate titles,
missing descriptions, missing/multiple H1s, headings, missing/multiple/invalid
canonicals and missing image alt attributes. Empty alt is valid for decoration.
WordPress detection is heuristic and uses only public markup indicators.

## Responsible network behavior

Audit only sites you are authorized to inspect. SiteForge respects matching
SiteForge or wildcard robots groups, Allow/Disallow wildcards and longest-match
precedence, and Crawl-delay. It refuses to crawl if robots cannot be retrieved;
404/410 mean no robots restrictions. Redirected robots policies fail closed.
A 401, 403, 429 or 503 stops new origin requests; concurrent requests already
in flight may finish. It does not retry, authenticate, solve challenges, or
bypass controls. External links and cross-origin redirects are not fetched.

DNS answers are checked before every request and pinned into that request's
client. Non-public addresses (including loopback, link-local, private, mapped
IPv4 and special-use ranges) are rejected conservatively. Implicit HTTP proxies
are disabled to preserve validated destination binding. This can prevent use
in environments that mandate an egress proxy. TLS verification stays enabled.
Only use `--allow-private` when you explicitly authorize access to the target
internal network, for example a local fixture server. Never use it to bypass
an access restriction.

`robots.txt` and `/sitemap.xml` statuses are reported. Sitemap inspection checks
for a recognizable XML root; it does not validate the XML schema, follow sitemap
indexes, or seed the crawl. HTML is not rendered with JavaScript. The crawler
checks same-origin anchor destinations; it does not audit external links,
images, scripts, or authenticated content. Canonicals are inspected, not fetched.
HTML output escapes all site content and disables scripts; CSV cells neutralize
spreadsheet formula prefixes. JSON includes the complete report; CSV contains
page rows, and terminal output is a concise summary.

## Development

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo build --locked --release
python3 scripts/verify-template.py
python3 -m unittest discover -s tests -v
```

CI runs
Rust checks on Windows, Linux and macOS. Tests use local fixtures and a loopback
HTTP server with explicit private-network authorization. No external website is
required. `src/network.rs` owns request safety, `robots.rs` policy, `html.rs`
analysis, `crawl.rs` orchestration, and `report.rs` rendering.

Read [AGENTS.md](AGENTS.md), [development standards](docs/development-standards.md),
and the [Rust profile](docs/technology-profiles/rust-cargo.md). Portable workflows
retain their [upstream provenance and license](.agents/skills/UPSTREAM.md).
SiteForge original code is MIT licensed; vendored Superpowers material retains
its existing attribution and MIT license.
