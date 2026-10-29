---
name: systematic-debugging
description: Use for unexpected behavior, test failures, startup failures, or integration errors.
---

# Systematic Debugging

Portable adaptation of obra/superpowers. Follow the repository AGENTS.md and the current user's instructions. [Original source](UPSTREAM.md) is preserved for attribution and comparison, not a second active workflow.

Reproduce the issue and read the complete relevant error. Inspect recent changes and runtime prerequisites. Trace data across boundaries without logging secrets. Compare working examples and identify differences. Form one evidence-based hypothesis; test it with the smallest reversible experiment. Fix the root cause and add a regression test when applicable. Rerun the reproduction and relevant suite. If repeated attempts fail, reassess the hypothesis or architecture rather than stacking guesses. Distinguish repository defects from missing services, permissions, and credentials. Supporting techniques: [root cause tracing](root-cause-tracing.md), [defense in depth](defense-in-depth.md), and [condition-based waiting](condition-based-waiting.md).

Upstream-only commands, hooks, browser scripts, and delegation are not prerequisites. See [provenance and adaptation policy](../UPSTREAM.md).
