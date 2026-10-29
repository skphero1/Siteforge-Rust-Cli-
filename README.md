# Sysfortech AI development starter

A reusable repository template for AI-assisted WordPress plugins and themes, Next.js, React, TypeScript, Python, Rust, CLI tools, and AI agents. This is engineering guidance and locally vendored skills, not an application or a global agent installation.

Start with [AGENTS.md](AGENTS.md), [development standards](docs/development-standards.md), and the [new project checklist](docs/new-project-checklist.md). Apply only the relevant [technology profile](docs/technology-profiles/README.md). [CLAUDE.md](CLAUDE.md) adapts the shared guidance; [agent compatibility](docs/agent-compatibility.md) records verified behavior and manual setup for other agents.

## Verify the template

Requires Git and Python 3.10+; no third-party Python packages or global installs.

```sh
python3 scripts/verify-template.py
python3 -m unittest discover -s tests -v
```

The validator checks required files, seven skill entry points and metadata, local Markdown references, the upstream pin, and original asset hashes. It does not certify instruction quality, automatic agent discovery, or application behavior. There is no application build, framework install, or service to start.

## Vendored skills

The seven portable workflows are brainstorming, writing-plans, test-driven-development, systematic-debugging, verification-before-completion, requesting-code-review, and finishing-a-development-branch. Read `.agents/skills/<name>/SKILL.md` explicitly if your agent does not discover them. [Provenance, adaptations, license, and update procedure](.agents/skills/UPSTREAM.md) document the exact official obra/superpowers commit. No Superpowers plugin, hook, browser helper, or subagent integration is installed or assumed.

## Enable the GitHub template

After reviewing and merging this branch into your chosen default branch, open the repository on GitHub, then **Settings → General → Template repository** and enable it. This requires repository administration permission; this task does not change the setting. Check the actual owner in the repository URL: the current checkout's origin is under `skphero1`, while Sysfortech is the intended project identity. Transfer or organization ownership is a separate administrator action.

For future projects, open this repository and choose **Use this template → Create a new repository**. Choose the owner, repository name, and visibility, then create and clone it. Usually copy only the default branch. A template copies files into a new repository with its own history; it does not establish automatic upstream synchronization. Follow the checklist and make project-specific changes in the new repository.

For an empty source repository, GitHub needs an initial/default branch before it can offer a normal pull request. Preserve the prepared feature branch if creating that base is not available; do not represent an uncreated PR as delivered.

## Scope and licensing

Third-party Superpowers material and adapted skills retain the [upstream MIT license](.agents/skills/LICENSE.superpowers). Select an appropriate license for original project material before wider distribution; no repository-wide license is implied by the vendored license.
