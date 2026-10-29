---
name: verification-before-completion
description: Use before claiming completion, committing, pushing, or preparing a pull request.
---

# Verification Before Completion

Portable adaptation of obra/superpowers. Follow the repository AGENTS.md and the current user's instructions. [Original source](UPSTREAM.md) is preserved for attribution and comparison, not a second active workflow.

Identify commands that prove each material claim. Run them on the final tree, read their output and exit status, and confirm that intended tests actually executed. Run applicable tests, build, lint, format check, and type check; report unavailable checks and reasons. Inspect the diff for unintended files and secrets. A zero-test result, running process, or successful install alone is not functional validation. Distinguish passed, failed, skipped, and unrun checks. State only observed results. Do not invent evidence, suppress failures, or treat another agent report as proof.

Upstream-only commands, hooks, browser scripts, and delegation are not prerequisites. See [provenance and adaptation policy](../UPSTREAM.md).
