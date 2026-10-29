# Agent compatibility and cloud evidence

Shared guidance is portable Markdown. A skill here is an instruction document, not executable software or a registered plugin. Reading and following it explicitly is the portable execution path. Its helper scripts are not required. Capability claims below are limited to this task environment, not every release of an agent.

| Agent / environment | Verified in this task | Manual setup / limitation |
| --- | --- | --- |
| Codex Cloud task | Shell file reads, Git, Python 3.12, and explicit loading of AGENTS.md and local skill entry points | Newly created skills are not in this turn's supplied skill catalog. Automatic discovery or refresh has not been demonstrated; explicitly read the selected SKILL.md. |
| Claude | CLAUDE.md adapter exists; runtime not tested | Ask the agent to read CLAUDE.md and follow AGENTS.md. Verify whether its installed version loads this file automatically. |
| Hermes | Portable files provided; runtime not tested | Point the project/session instructions at AGENTS.md and explicitly load a relevant skill. Verify config syntax with the installed version's documentation. |
| OpenCode | Portable files provided; runtime not tested | Configure repository instructions using the installed version's documented mechanism, then verify an actual read of AGENTS.md and one skill. |
| Pi | Portable files provided; runtime not tested | Supply AGENTS.md and selected skill paths in the project/session context; verify its version's loader before relying on discovery. |
| Other agents | Markdown can be supplied as context | Use an explicit read prompt; do not assume a particular config file, plugin namespace, hook, or delegation API. |

Suggested manual instruction: "Read AGENTS.md, then read .agents/skills/verification-before-completion/SKILL.md and apply it before reporting results. Follow only the technology profile that matches this project."

## Validation protocol

1. In the actual target agent, read AGENTS.md and the selected skill file. Ask it to state the relevant constraints to confirm it used the content.
2. Perform a small real task: run `python3 scripts/verify-template.py`, read the complete output and exit status, and report observed results using the verification skill.
3. Run `python3 -m unittest discover -s tests -v` to verify the checker rejects damaged templates. This tests structure, not model behavior or automatic discovery.
4. Record which loading path was used, the commands/results, and unsupported capabilities. Repeat in a new session before claiming cross-session discovery.

## Current cloud task

The agent explicitly read the root instructions and the local verification and branch-finishing adapters through shell file reads. The applicable constraints are evidence before claims, no automatic worktrees, preserving user work, feature-branch delivery, and no unapproved deployment or publishing. The actual execution path is sequential work and command execution in this session, not a plugin dispatcher. The validator and negative-case suite are the functional skill exercise; final observed results are recorded in the task delivery report.

No Superpowers plugin, hook, automatic skill-discovery integration, browser companion, independent reviewer, or cross-agent runtime has been installed or validated. No account-wide Codex settings were changed. Original upstream assets are retained for provenance only. No application exists, so framework builds, service startup, and technology-specific tests are not applicable yet.

GitHub HTTPS Git reads worked through the managed proxy when run outside the command sandbox. Restricted sandbox network calls could not reach that proxy. GitHub CLI/API and push access must be tested separately: working Git reads alone do not prove those capabilities. Do not print tokens or replace injected authentication based only on a login-status warning.

Observed template checks in this task: `python3 scripts/verify-template.py` passed; `python3 -m unittest discover -s tests -v` executed eight tests and passed. The negative cases cover missing required files, bad skill names/descriptions, broken or escaping file references, an altered upstream asset, and an invalid pin. This is explicit-read skill execution, not proof of automatic skill discovery. The first test run failed because the validator had not yet been implemented; the completed implementation passed all eight cases.

A read-only GitHub API request with the inherited CLI authentication returned `Forbidden`, so API-based PR creation is not established. The current network preset includes GitHub Git access but does not list api.github.com. Any needed API allowlist or authorization change belongs in environment settings; never bypass the proxy. Git delivery is checked independently.
