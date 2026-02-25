## 11. Handover

### Implementation Status

| Phase | Description | Status |
|-------|-------------|--------|
| Phase 1 | Foundation (Tauri, PostgreSQL, Signal capture, hotkey) | Complete |
| Phase 2 | CRUD Operations (all 5 object repositories + services) | In Progress |
| Phase 3 | Lifecycle Enforcement (state machines, role checks) | Planned |
| Phase 4 | Frontend Views (issues, decisions, artifacts, dashboard) | Planned |
| Phase 5 | Offline Support (SQLite write-behind buffer) | Optional |

### What Works

- Tauri v2 project builds and launches
- PostgreSQL connection with sqlx
- Schema with 11 tables + 7 views + append-only triggers
- Signal capture via IPC (capture_signal command)
- Global hotkey Ctrl+Alt+Z toggles capture window
- Frontend signal capture UI
- 23 IPC command registrations in main.rs

### Known Issues

- Repository and service modules are stubs (signatures exist, implementation pending)
- Frontend views beyond signal capture are planned but not built
- Lifecycle enforcement exists as module structure but not yet wired
- No test suite yet
- No CI/CD pipeline

### Critical Constraints

1. **Never add UPDATE/DELETE to the repository layer** — append-only is a design invariant
2. **Never bypass role checks** — Steward authority is enforced, not suggested
3. **Never close an Issue without a verified Artifact** — `close_requires_artifact` is non-negotiable
4. **Never store secrets in frontend** — all DB access goes through Tauri IPC backend

### Next Priorities

1. Complete repository module implementations (append_issue, append_decision, etc.)
2. Wire service layer with business logic
3. Implement lifecycle state machine enforcement
4. Build frontend views for Issue management
5. Add test coverage for IPC commands and lifecycle transitions

### Dev Quickref

```bash
# Setup
createdb zfss
psql -d zfss -f migrations/001_initial_schema.sql
psql -d zfss -f migrations/002_append_only_enforcement.sql
psql -d zfss -f migrations/003_signal_link_events.sql
echo 'ZFSS_DATABASE_URL=postgresql://localhost/zfss' > .env

# Development
npm install
npm run tauri dev

# Build
npm run tauri build
```
