## 10. Ecosystem Integration

### Forge Ecosystem Position

ZFSS occupies the **feedback boundary** of the Forge ecosystem. It captures external user feedback (Signals) and metabolizes them through a governed lifecycle into verified outcomes (Artifacts, Responses).

### DataForge Authority Model

ZFSS follows the Forge ecosystem's authority doctrine:
- **Local PostgreSQL is the source of truth** (migrated from Render cloud)
- No cloud service holds authoritative state
- The database is the contract — append-only semantics enforced at the trigger level

### Shared Patterns

| Pattern | ZFSS Implementation |
|---------|---------------------|
| Append-only writes | PostgreSQL triggers + repository layer |
| Lifecycle state machines | Signal, Issue, Response state enums |
| Role-based authority | 4 roles (Steward, Operator, Engineer, AI) |
| Typed IDs | Prefixed UUIDs (`sig_*`, `iss_*`, etc.) |
| Tauri v2 desktop | Same framework as Forge:SMITH and ForgeCommand |

### Render-to-Local Migration

ZFSS was originally deployed on Render (cloud PostgreSQL). The authority was cut over to local PostgreSQL with:
- Export tooling (`scripts/export_render_snapshot.sh`)
- Import tooling (`scripts/import_snapshot_to_local.sh`)
- Verification (`scripts/verify_migration.py` — row count comparison)
- Credential rotation and cloud service disconnection

See `docs/local_postgres_authority.md` for the full cutover documentation.

### Future Integration Points

| Service | Integration | Status |
|---------|-------------|--------|
| ForgeCommand | Orchestration of ZFSS health checks | Planned |
| DataForge | Centralized Signal/Issue analytics | Planned |
| NeuroForge | AI-assisted Signal triage and classification | Planned |
| BugCheck | Signal-to-Issue correlation with bug findings | Planned |
