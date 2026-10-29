---
name: finishing-a-development-branch
description: Use after implementation to verify and deliver a feature branch.
---

# Finishing A Development Branch

Portable adaptation of obra/superpowers. Follow the repository AGENTS.md and the current user's instructions. [Original source](UPSTREAM.md) is preserved for attribution and comparison, not a second active workflow.

Run [verification before completion](../verification-before-completion/SKILL.md) on the final tree. Inspect status and preserve unrelated work. Determine the actual remote and base branch rather than assuming an owner or main branch. Commit on a feature branch with a descriptive message. If the user authorized delivery, push the branch and prepare a pull request using available tooling; do not ask again for a choice already made. If access or a base branch is missing, report the exact blocker and preserve the local commit. Never force-push, merge, deploy, publish, or delete branches/worktrees without applicable authorization. In managed cloud tasks use the existing checkout; do not create a worktree unless explicitly requested.

Upstream-only commands, hooks, browser scripts, and delegation are not prerequisites. See [provenance and adaptation policy](../UPSTREAM.md).
