# Next.js / React / TypeScript

Apply only when package manifests identify this stack. Respect the package manager, lockfile, Node engine, router architecture, and framework versions already selected. Use frozen installs such as `npm ci` when a matching lock exists. Consult documentation matching the installed stable version; do not assume deprecated framework lint commands exist.

Use strict TypeScript, explicit server/client boundaries, accessible semantic components, and appropriate loading/error states. Keep secrets server-side; public-prefixed environment variables are exposed to clients. Validate input and enforce authorization on the server. Review caching and rendering choices for data isolation. Avoid premature state libraries and abstractions.

Run actual package scripts for tests, lint, type check, format check, and production build. Use behavior-focused component tests and representative end-to-end checks when configured. Start the documented server and test a meaningful route when runtime behavior matters. Document external API and environment requirements without embedding credentials. Do not interpret successful compilation as proof of authorization or runtime behavior.
