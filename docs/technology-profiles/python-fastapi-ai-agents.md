# Python / FastAPI / AI agents

Apply only to Python projects. Use the declared Python version, an isolated virtual environment, and the existing manager and lockfile. Do not install project dependencies globally. Follow pyproject.toml and existing import, packaging, async, and typing conventions.

For FastAPI, validate request/response models, enforce authentication and authorization, handle exceptions safely, and avoid blocking the event loop. Test lifecycle and dependency boundaries; use disposable databases and explicit migrations. Never point tests at production data.

For agents, treat prompts, retrieved content, tool output, and model output as untrusted data. Bound tool permissions, timeouts, retries, spending, and execution scope. Require human authorization for consequential external actions. Keep secrets out of prompts and traces. Use deterministic fake providers for routine tests and clearly separated, opt-in live-provider tests; do not claim mocked tests validate live model behavior.

Run the configured pytest suite, Ruff or other project lint/format checks, and mypy or pyright where configured. Exercise API routes and failure cases through the project's supported test client. Report skipped live tests and required secure credential bindings separately.
