---
name: requesting-code-review
description: Use before delivery or integration of a substantial change.
---

# Requesting Code Review

Portable adaptation of obra/superpowers. Follow the repository AGENTS.md and the current user's instructions. [Original source](UPSTREAM.md) is preserved for attribution and comparison, not a second active workflow.

Prepare a review brief: requirements, scope, changed files, verification results, risks, and base/head commits when available. Use [the reviewer checklist](code-reviewer.md) for correctness, security, maintainability, and test coverage. Request an independent human or agent review only through available, authorized mechanisms. If no independent reviewer is available, perform a clearly labeled self-review; never describe it as independent. Do not assume a subagent tool or model exists. Resolve critical and important findings, rerun affected checks, and record remaining limitations. Creating a pull request does not mean review or approval occurred.

Upstream-only commands, hooks, browser scripts, and delegation are not prerequisites. See [provenance and adaptation policy](../UPSTREAM.md).
