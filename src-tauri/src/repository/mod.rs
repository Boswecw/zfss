//! ZFSS Repository Layer
//!
//! Provides append-only access patterns for canonical objects so callers never issue UPDATE/DELETE
//! on `issues`, `signals`, `decisions`, `artifacts`, or `responses`.

use crate::models::ids::SignalId;
use crate::models::{Signal, SignalCreate, SignalStatus};
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;

/// Insert a new signal and record its initial status in `signal_status_history`.
pub async fn append_signal(pool: &PgPool, input: SignalCreate, created_by: &str) -> Result<Signal> {
    let signal_id = SignalId::new();
    let environment = to_json_value(input.environment)?;
    let reporter = to_json_value(input.reporter)?;
    let mut tx = pool
        .begin()
        .await
        .context("begin signal append transaction")?;

    sqlx::query(
        "INSERT INTO signals (id, source, raw_text, app_key, app_version, environment, reporter, status, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 'new', $8)",
    )
    .bind(signal_id.as_str())
    .bind(input.source.as_str())
    .bind(input.raw_text)
    .bind(input.app_key)
    .bind(input.app_version)
    .bind(environment)
    .bind(reporter)
    .bind(created_by)
    .execute(&mut *tx)
    .await
    .context("failed to insert signal")?;

    sqlx::query(
        "INSERT INTO signal_status_history (signal_id, new_status, changed_by, reason)
        VALUES ($1, 'new', $2, 'Initial capture')",
    )
    .bind(signal_id.as_str())
    .bind(created_by)
    .execute(&mut *tx)
    .await
    .context("failed to insert signal status history for new signal")?;

    tx.commit()
        .await
        .context("commit signal insert transaction")?;

    get_signal(pool, signal_id.as_str()).await?.ok_or_else(|| {
        anyhow!(
            "signal {} was inserted but could not be loaded afterwards",
            signal_id.as_str()
        )
    })
}

/// List signals, optionally filtered by lifecycle status, ordered newest first while deriving status/link state from history tables.
pub async fn list_signals(
    pool: &PgPool,
    status_filter: Option<SignalStatus>,
    limit: i64,
) -> Result<Vec<Signal>> {
    let limit = limit.clamp(1, 100);
    let status_value = status_filter.map(|s| s.as_str().to_string());

    let rows = sqlx::query_as::<_, SignalRow>(
        r#"
        SELECT
            s.id,
            s.source,
            s.raw_text,
            s.app_key,
            s.app_version,
            s.environment,
            s.reporter,
            COALESCE(latest_status.new_status, s.status) AS status,
            latest_link.issue_id AS linked_issue_id,
            s.created_at,
            s.created_by
        FROM signals s
        LEFT JOIN LATERAL (
            SELECT new_status
            FROM signal_status_history h
            WHERE h.signal_id = s.id
            ORDER BY h.changed_at DESC, h.id DESC
            LIMIT 1
        ) AS latest_status ON TRUE
        LEFT JOIN LATERAL (
            SELECT issue_id
            FROM signal_links l
            WHERE l.signal_id = s.id
            ORDER BY l.linked_at DESC, l.id DESC
            LIMIT 1
        ) AS latest_link ON TRUE
        WHERE ($1::text IS NULL OR COALESCE(latest_status.new_status, s.status) = $1)
        ORDER BY s.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(status_value.as_deref())
    .bind(limit)
    .fetch_all(pool)
    .await
    .context("failed to query signals")?;

    Ok(rows.into_iter().map(|row| row.into_signal()).collect())
}

/// Fetch the detailed signal record, including the latest status and linked issue (via append-only tables).
pub async fn get_signal(pool: &PgPool, signal_id: &str) -> Result<Option<Signal>> {
    let row = sqlx::query_as::<_, SignalRow>(
        r#"
        SELECT
            s.id,
            s.source,
            s.raw_text,
            s.app_key,
            s.app_version,
            s.environment,
            s.reporter,
            COALESCE(latest_status.new_status, s.status) AS status,
            latest_link.issue_id AS linked_issue_id,
            s.created_at,
            s.created_by
        FROM signals s
        LEFT JOIN LATERAL (
            SELECT new_status
            FROM signal_status_history h
            WHERE h.signal_id = s.id
            ORDER BY h.changed_at DESC, h.id DESC
            LIMIT 1
        ) AS latest_status ON TRUE
        LEFT JOIN LATERAL (
            SELECT issue_id
            FROM signal_links l
            WHERE l.signal_id = s.id
            ORDER BY l.linked_at DESC, l.id DESC
            LIMIT 1
        ) AS latest_link ON TRUE
        WHERE s.id = $1
        "#,
    )
    .bind(signal_id)
    .fetch_optional(pool)
    .await
    .context("failed to fetch signal")?;

    Ok(row.map(|row| row.into_signal()))
}

/// Link a signal to an issue by recording an append-only history entry and logging the link event.
pub async fn link_signal_to_issue(
    pool: &PgPool,
    signal_id: &str,
    issue_id: &str,
    actor: &str,
) -> Result<Signal> {
    let mut tx = pool
        .begin()
        .await
        .context("begin signal link transaction")?;

    let signal_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM signals WHERE id = $1)")
            .bind(signal_id)
            .fetch_one(&mut *tx)
            .await
            .context("failed to verify signal existence")?;

    if !signal_exists {
        return Err(anyhow!("Signal not found: {}", signal_id));
    }

    let issue_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM issues WHERE id = $1)")
            .bind(issue_id)
            .fetch_one(&mut *tx)
            .await
            .context("failed to verify issue existence")?;

    if !issue_exists {
        return Err(anyhow!("Issue not found: {}", issue_id));
    }

    let current_status: Option<String> = sqlx::query_scalar(
        r#"
        SELECT COALESCE(
            (
                SELECT new_status
                FROM signal_status_history h
                WHERE h.signal_id = $1
                ORDER BY h.changed_at DESC, h.id DESC
                LIMIT 1
            ),
            (
                SELECT status
                FROM signals
                WHERE id = $1
            )
        )
        "#,
    )
    .bind(signal_id)
    .fetch_one(&mut *tx)
    .await
    .context("failed to read current signal status")?;

    sqlx::query(
        "INSERT INTO signal_status_history (signal_id, old_status, new_status, changed_by, reason)
        VALUES ($1, $2, 'linked', $3, 'Linked to issue via UI')",
    )
    .bind(signal_id)
    .bind(current_status.as_deref())
    .bind(actor)
    .execute(&mut *tx)
    .await
    .context("failed to append signal status history for link")?;

    sqlx::query(
        "INSERT INTO signal_links (signal_id, issue_id, linked_by, reason)
        VALUES ($1, $2, $3, 'Linked via desktop application')",
    )
    .bind(signal_id)
    .bind(issue_id)
    .bind(actor)
    .execute(&mut *tx)
    .await
    .context("failed to insert signal link event")?;

    sqlx::query(
        "INSERT INTO audit_log (event_type, entity_type, entity_id, actor, actor_role, action, details)
        VALUES ('signal', 'signal_link', $1, $2, 'Operator', 'link', $3)",
    )
    .bind(signal_id)
    .bind(actor)
    .bind(serde_json::json!({
        "target_issue": issue_id,
        "reason": "Linked from desktop UI"
    }))
    .execute(&mut *tx)
    .await
    .context("failed to record audit trail for link")?;

    tx.commit()
        .await
        .context("commit signal link transaction")?;

    get_signal(pool, signal_id)
        .await?
        .ok_or_else(|| anyhow!("signal {} disappeared after linking", signal_id))
}

fn to_json_value<T>(value: Option<T>) -> Result<Option<Value>>
where
    T: Serialize,
{
    value
        .map(|inner| serde_json::to_value(inner).context("serialize JSON field"))
        .transpose()
}

#[derive(sqlx::FromRow)]
struct SignalRow {
    id: String,
    source: String,
    raw_text: String,
    app_key: Option<String>,
    app_version: Option<String>,
    environment: Option<Value>,
    reporter: Option<Value>,
    status: String,
    linked_issue_id: Option<String>,
    created_at: DateTime<Utc>,
    created_by: String,
}

impl SignalRow {
    fn into_signal(self) -> Signal {
        Signal {
            id: self.id,
            source: self.source,
            raw_text: self.raw_text,
            app_key: self.app_key,
            app_version: self.app_version,
            environment: json_to(self.environment),
            reporter: json_to(self.reporter),
            status: self.status,
            linked_issue_id: self.linked_issue_id,
            created_at: self.created_at,
            created_by: self.created_by,
        }
    }
}

fn json_to<T>(value: Option<Value>) -> Option<T>
where
    T: DeserializeOwned,
{
    value.and_then(|v| serde_json::from_value(v).ok())
}
