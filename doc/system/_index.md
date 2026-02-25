# ZFSS — System Documentation

**Document version:** 1.0 (2026-02-25)

> Zen Feedback & Service System — append-only feedback metabolism for the Forge ecosystem.
> "Capture. Triage. Decide. Prove. Respond."

| Part | File | Contents |
|------|------|----------|
| §1 | `01-overview-philosophy.md` | Service identity, append-only doctrine, ecosystem role |
| §2 | `02-architecture.md` | Tauri v2 IPC bridge, repository pattern, data flow |
| §3 | `03-tech-stack.md` | Rust 2024, TypeScript 5, Tauri 2, sqlx, dependencies |
| §4 | `04-project-structure.md` | Dual source trees (src-tauri/ + src/), module map |
| §5 | `05-config-env.md` | Environment variables, settings, device ID persistence |
| §6 | `06-frontend.md` | Signal capture UI, views, router, API layer |
| §7 | `07-tauri-commands.md` | 23 IPC commands across 5 canonical object domains |
| §8 | `08-backend-internals.md` | Models, lifecycle state machines, repository, roles |
| §9 | `09-database-schema.md` | 11 tables, 7 views, 3 migrations, append-only triggers |
| §10 | `10-ecosystem-integration.md` | DataForge authority, Forge design patterns, future hooks |
| §11 | `11-handover.md` | Implementation status, known issues, next priorities |

## Quick Assembly

```bash
bash doc/system/BUILD.sh   # Assembles all parts into doc/zsSYSTEM.md
```

*Last updated: 2026-02-25*
