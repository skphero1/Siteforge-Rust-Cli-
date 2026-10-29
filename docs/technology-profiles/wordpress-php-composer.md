# WordPress / PHP / Composer

Apply only to WordPress plugins, themes, or PHP projects. Determine supported WordPress and PHP versions from requirements and metadata; pin a compatible local runtime. Respect Composer constraints and commit the appropriate lockfile. Use `composer install` for an existing lock; never update dependencies as incidental setup.

Follow WordPress coding standards and existing namespace, prefix, autoloading, translation, and packaging conventions. Check capabilities as well as nonces; nonces alone are not authorization. Validate and sanitize input, escape output for its context, use prepared database queries, and define REST permission callbacks. Never edit WordPress core or deploy against a live site for testing.

Use project-defined PHPUnit/integration tests, PHPCS with the configured WordPress standard, PHP syntax checks, and static analysis where configured. Test against a disposable WordPress/database setup when APIs require it. Plugins need activation, upgrade, and uninstall coverage without accidental data loss; themes need template, escaping, asset, accessibility, and supported editor checks. Run Node asset checks only when the project has an asset toolchain. Report integration tests blocked by missing WordPress or database services.
