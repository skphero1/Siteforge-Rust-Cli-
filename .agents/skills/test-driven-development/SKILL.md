---
name: test-driven-development
description: Use when adding or changing executable behavior or fixing a bug.
---

# Test Driven Development

Portable adaptation of obra/superpowers. Follow the repository AGENTS.md and the current user's instructions. [Original source](UPSTREAM.md) is preserved for attribution and comparison, not a second active workflow.

Write a focused test of observable behavior before implementation. Run it and confirm failure for the intended missing behavior, not a syntax or environment error. Make the smallest maintainable change that passes. Run the focused test and relevant regression suite, then refactor while keeping them green. Include boundary, invalid-input, and security cases where relevant. Avoid tests that assert only mock calls or mirror implementation. See [writing good tests](writing-good-tests.md). For documentation and configuration, use meaningful structural or functional checks rather than artificial application tests. Preserve existing user work; never delete code merely to reenact a test-first sequence. Report if a test-first sequence was not performed.

Upstream-only commands, hooks, browser scripts, and delegation are not prerequisites. See [provenance and adaptation policy](../UPSTREAM.md).
