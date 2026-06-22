## 1. Overview & Philosophy

### Identity

**ZFSS** (Zen Feedback & Service System) is a Tauri v2 desktop application that captures, triages, and responds to user feedback with append-only PostgreSQL as the authoritative data store.

- **Purpose:** Feedback metabolism — turning raw user signals into verified learning artifacts
- **Paradigm:** Append-only, lifecycle-governed, role-based authority
- **Deployment:** Local desktop app (Tauri), local PostgreSQL
- **Status:** Phase 1 complete, Phase 2 in progress

### Design Commitments

1. **DataForgeDB (local PostgreSQL) is authoritative** — single source of truth, no cloud authority
2. **Append-only semantics** — no UPDATE/DELETE on canonical records, enforced at database level via triggers
3. **Cloud services are stateless consumers** — can only read or submit new Signals
4. **SQLite is optional** — only as write-behind buffer for offline Signal capture
5. **Lifecycle enforced in code** — no Issue may close without verified Artifact
6. **Role-based authority** — Steward decides, Operator executes, Engineer builds, AI suggests

### Canonical Objects

| Object | ID Pattern | Purpose |
|--------|------------|---------|
| Signal | `sig_*` | Raw immutable user expression |
| Issue | `iss_*` | System's understanding (many Signals → one Issue) |
| Decision | `dec_*` | Declared intent (append-only, can supersede) |
| Artifact | `art_*` | Proof of learning (verified by Steward) |
| Response | `rsp_*` | Controlled outward communication |

### Ecosystem Role

ZFSS sits at the feedback boundary of the Forge ecosystem. User-facing signals flow in, get triaged into Issues, receive Decisions, produce Artifacts (proof of learning), and generate controlled Responses back to users. The append-only model ensures a complete audit trail of every feedback interaction.
