# New project checklist

1. After this starter is merged into the intended default branch, enable GitHub's Template repository setting as described in README.md.
2. Use the template to create a new repository with the intended owner, name, and visibility. Clone that repository and verify its origin before pushing.
3. Replace starter identity and documentation with the project's purpose, supported versions, architecture, acceptance criteria, and local commands. Choose a license for original project material; preserve third-party notices.
4. Keep AGENTS.md and the concise CLAUDE.md adapter. Select only relevant technology profiles. Remove irrelevant profiles if helpful; update links and validator expectations if changing the template structure.
5. Choose runtime versions and the ecosystem's package manager. Add an application only as a separate, explicitly requested project task. Commit manifests and suitable lockfiles.
6. Add safe example configuration, local services, and credential requirements. Store real secrets in secure environment or CI settings, never in the repository.
7. Add project tests, build/lint/format/type-check commands, and CI using least privilege. Run them locally. Template validation alone does not validate application behavior.
8. For the chosen agent, follow the manual compatibility setup and verify it reads instructions and one selected skill. Do not assume plugins or discovery transfer from another machine.
9. Develop on a feature branch, review the diff, run checks, commit, and prepare a PR. Document deployment and publishing as separately authorized actions.
