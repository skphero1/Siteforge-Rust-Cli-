# Changelog

## 0.1.0

- Same-origin asynchronous crawling with bounded concurrency, page/frontier/body
  limits, request pacing, timeouts and manual redirect chains.
- Public-destination checks with per-request DNS pinning and explicit private
  network authorization; no automatic authentication or access-control bypass.
- Robots policy enforcement and crawl-delay, origin stop on denial/rate limits,
  and robots/sitemap discovery reporting.
- Titles, descriptions, headings, canonicals, missing alt attributes, public
  WordPress indicators, HTTP errors and broken anchor destination reporting.
- Terminal, JSON, CSV and escaped standalone HTML output.
- Local fixture tests and Linux/Windows/macOS CI.
