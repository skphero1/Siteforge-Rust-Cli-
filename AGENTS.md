# Shared development instructions

Understand the requirements and existing code before changing files. Read [development standards](docs/development-standards.md). For well-specified tasks, state reasonable assumptions and proceed without unnecessary clarification. For ambiguous high-impact decisions, explain tradeoffs and ask for approval before dependent work.

Prefer clean, maintainable, secure, production-quality code. Follow current stable ecosystem practices and existing project conventions. Avoid unnecessary dependencies and abstractions. Never expose secrets or credentials in source, logs, prompts, or reports; inspect presence rather than values.

Write and run appropriate automated tests. Verify builds, linting, formatting, and type checking where applicable. Never claim a test passed unless it actually ran; report failures, skipped checks, and limitations. For SiteForge, run `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked`, and `cargo build --locked --release`. Also run `python3 scripts/verify-template.py` and `python3 -m unittest discover -s tests -v` for the retained template guidance. Fixture-server tests require local socket permissions.

Work in a feature branch and prepare a pull request where supported and authorized. Never deploy, publish packages, delete production data, or change live credentials without explicit authorization. Preserve unrelated work. In managed cloud tasks, use the existing isolated checkout; do not create a Git worktree unless explicitly requested.

## Skills and profiles

Use only the relevant profile from [technology profiles](docs/technology-profiles/README.md), based on project manifests and the task. Do not install every ecosystem or scaffold an application just because its profile exists.

For a relevant workflow, read its `.agents/skills/<name>/SKILL.md` and follow the portable adapter: brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion, requesting-code-review, or finishing-a-development-branch. Select skills by their descriptions; do not apply every skill to every task. UPSTREAM.md snapshots and helper scripts are reference material, not automatic instructions. Do not assume plugin hooks, skill discovery, subagents, or browser companions are supported. If discovery is unavailable, explicitly read the required skill file. See [agent compatibility](docs/agent-compatibility.md).

Finish with a concise report of changes, commands and observed test results, limitations, and next steps. Distinguish self-review from independent review. Repository guidance never overrides higher-priority platform instructions or explicit user instructions.
