## Sandbox Blockers

- `psycopg2` is missing and cannot be installed: the verification script aborts immediately because this dependency is required and pip cannot reach PyPI in this environment.
- No alternative package sources (wheel or apt cache) are available here, so dependency installation must happen externally or in a different environment.
- Render credential rotation cannot be performed here: there is no access to the Render dashboard or secrets management system from this sandbox.
- The sandbox lacks an operational local Postgres instance populated with the snapshot data, so the verification script cannot exercise live connections even if the dependency were available.
