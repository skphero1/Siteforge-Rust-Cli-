# Development standards

Start with the task's acceptance criteria, existing code, manifests, lockfiles, CI, and project-specific instructions. Keep changes scoped and favor existing conventions and stable supported tooling. Inspect dependency maintenance and security before adding packages. Pin toolchains and use lockfile-respecting installation where supported.

For multi-step work, record affected files, constraints, test cases, and verification commands. Ask about unresolved high-impact choices; proceed on clear requirements. Implement behavior in small testable steps. Use test-first development for behavior changes and regression tests for bugs. Documentation and configuration need meaningful checks, not artificial application scaffolding.

Validate boundaries, authorization, error handling, and untrusted inputs. Use least privilege and safe defaults. Keep secrets outside version control; example environment files contain placeholders only. Never log credentials, customer data, or model prompts containing sensitive information. Review AI-generated output as critically as human code.

Run the relevant automated tests and applicable build, lint, format, and type checks. Use repository scripts rather than assuming generic commands exist. Preserve actual exit statuses. Record what was executed, what failed, and what could not run. Review diffs for correctness, security, maintainability, and accidental generated files before committing.

Use a descriptive feature branch and a reviewable commit. PR descriptions explain the problem, resulting behavior, validation, and material limitations. Do not merge, deploy, publish, delete production data, or rotate live credentials without authorization. A request to create a PR is not authorization to merge it.

SiteForge is a Cargo application with a committed lockfile and local HTTP fixture tests. Run the Rust and Python checks in AGENTS.md. No persistent services or credentials are required. Template checks validate retained guidance only; they do not establish application readiness.
