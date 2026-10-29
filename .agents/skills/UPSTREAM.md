# Superpowers provenance

Source: https://github.com/obra/superpowers
Pinned commit: `2ce8aa15d05bd57213e6f746f953aafb7ded7a8b` (reviewed 2026-10-29).
License: [MIT, copyright 2025 Jesse Vincent](LICENSE.superpowers).

Seven selected directories are vendored with all supporting files. Original skill entry points are renamed UPSTREAM.md; their content and all helper assets remain unchanged and are hash-locked in upstream-lock.json. Active SKILL.md files are locally adapted derivatives, not verbatim upstream skills. The upstream license applies to these derivatives. No broader license is asserted for this repository's original material.

Adaptations remove mandatory plugin namespaces, automatic delegation, worktree creation, browser setup, redundant approval menus, and destructive test-first cleanup. Clear requirements proceed directly; ambiguous high-impact choices need approval. Review can fall back to an explicitly labeled self-review. Planning executes in the current session. Documentation uses structural checks. Retained browser scripts are reference assets: not launched, audited for hosting, or required by this template. Original files can mention omitted upstream skills; those references are historical and not active dependencies.

To update: fetch the official repository over verified HTTPS, choose an exact commit, inspect all seven directories and license, replace original snapshots/support files, review each local adapter, regenerate original-file hashes and the pin, and rerun validation and its negative tests. Do not replace adapters blindly or execute upstream hooks. No runtime download or global installation is needed.
