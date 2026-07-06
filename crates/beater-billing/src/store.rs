//! Persistence for billing: a [`BillingStore`] trait and a SQLite
//! implementation mirroring `beater-usage`'s `SqliteUsageLedger` and the
//! `beater-store-sql` idiom (an `Arc<Mutex<Connection>>`, JSON row payloads,
//! `IntoStoreResult` error mapping).
//!
//! Robustness here:
//! * subscription plan changes are **optimistic** — `UPDATE ... WHERE version = ?`
//!   inside a transaction, so a lost update affects zero rows and is reported as
//!   [`StoreError::Conflict`] (which the service maps to
//!   `ConcurrentModification`); the proration adjustments are written in the
//!   same transaction, so the plan switch and its ledger entries are atomic.
//! * invoices are **insert-if-absent** on `(org, period_key)` so rollup can
//!   never double-create.
//! * adjustments and Stripe events are **append-only**; the Stripe event table
//!   has a `UNIQUE` id so at-least-once delivery dedups to exactly-once.

use crate::{
    AdjustmentKind, AetherSettlementReceipt, BillingAdjustment, Invoice, InvoiceStatus, Plan,
    PlanId, Subscription, SubscriptionStatus,
};
use async_trait::async_trait;
use beater_core::OrganizationId;
use beater_store::{StoreError, StoreResult};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Persistence contract for billing state. All money lives inside the JSON
/// payloads as [`Money`](beater_core::Money) (integer micros); SQL only carries
/// keys and status.
#[async_trait]
pub trait BillingStore: Send + Sync {
    // --- plans ---------------------------------------------------------------
    async fn put_plan(&self, plan: Plan) -> StoreResult<()>;
    async fn get_plan(&self, plan_id: &PlanId) -> StoreResult<Option<Plan>>;
    async fn list_plans(&self) -> StoreResult<Vec<Plan>>;

    // --- subscriptions -------------------------------------------------------
    /// Insert a new subscription. Fails with [`StoreError::Conflict`] if the org
    /// already has one.
    async fn create_subscription(&self, subscription: Subscription) -> StoreResult<Subscription>;
    async fn get_subscription(&self, org_id: &OrganizationId) -> StoreResult<Option<Subscription>>;
    /// Optimistic plan change: bump the plan only if `expected_version` still
    /// matches, appending `adjustments` atomically. Returns
    /// [`StoreError::Conflict`] on a lost update (no rows affected).
    async fn change_subscription_plan(
        &self,
        org_id: &OrganizationId,
        expected_version: i64,
        new_plan_id: &PlanId,
        adjustments: &[BillingAdjustment],
    ) -> StoreResult<Subscription>;
    /// Force a subscription status (used when applying ordered Stripe events).
    /// Bumps `version`. Returns `None` if the org has no subscription.
    async fn set_subscription_status(
        &self,
        org_id: &OrganizationId,
        status: SubscriptionStatus,
    ) -> StoreResult<Option<Subscription>>;

    // --- invoices ------------------------------------------------------------
    /// Insert-if-absent on `(org, period_key)`. Returns the stored invoice,
    /// whether freshly inserted or already present.
    async fn insert_invoice_if_absent(&self, invoice: Invoice) -> StoreResult<Invoice>;
    async fn get_invoice(
        &self,
        org_id: &OrganizationId,
        period_key: &str,
    ) -> StoreResult<Option<Invoice>>;
    async fn list_invoices(&self, org_id: &OrganizationId) -> StoreResult<Vec<Invoice>>;
    /// Idempotent finalize: `Draft -> Finalized`; any other status is returned
    /// unchanged. Never mutates a finalized invoice's amounts.
    async fn finalize_invoice(
        &self,
        org_id: &OrganizationId,
        period_key: &str,
    ) -> StoreResult<Invoice>;
    /// Transition an invoice's status (e.g. `Finalized -> Paid`/`Void`). Amounts
    /// are never touched.
    async fn set_invoice_status(
        &self,
        org_id: &OrganizationId,
        period_key: &str,
        status: InvoiceStatus,
    ) -> StoreResult<Invoice>;

    // --- adjustments (append-only) ------------------------------------------
    async fn append_adjustment(
        &self,
        adjustment: BillingAdjustment,
    ) -> StoreResult<BillingAdjustment>;
    async fn list_adjustments(
        &self,
        org_id: &OrganizationId,
    ) -> StoreResult<Vec<BillingAdjustment>>;

    // --- Aether settlement dedup / invoice payment --------------------------
    /// Atomically record an Aether settlement receipt, mark its finalized invoice
    /// paid, and append the corresponding immutable adjustment. Returns `false`
    /// if the same settlement or payment hash was already recorded.
    async fn apply_aether_settlement(
        &self,
        receipt: &AetherSettlementReceipt,
        adjustment: &BillingAdjustment,
    ) -> StoreResult<bool>;

    // --- Stripe event dedup / ordering --------------------------------------
    /// Record a Stripe event id (UNIQUE). Returns `true` if it was newly
    /// inserted, `false` if it had already been seen (duplicate delivery).
    async fn record_stripe_event(
        &self,
        event_id: &str,
        object_id: &str,
        created: i64,
    ) -> StoreResult<bool>;
    /// The `created` timestamp of the newest *applied* event for `object_id`.
    async fn last_applied_stripe_created(&self, object_id: &str) -> StoreResult<Option<i64>>;
    /// Mark a recorded event as applied (drives out-of-order detection).
    async fn mark_stripe_event_applied(&self, event_id: &str) -> StoreResult<()>;
}

/// SQLite-backed [`BillingStore`].
#[derive(Clone)]
pub struct SqliteBillingStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteBillingStore {
    pub fn in_memory() -> anyhow::Result<Self> {
        let connection = Connection::open_in_memory()?;
        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
        };
        store.init()?;
        Ok(store)
    }

    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let connection = Connection::open(path)?;
        let store = Self {
            connection: Arc::new(Mutex::new(connection)),
        };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> anyhow::Result<()> {
        let connection = beater_store::lock_poisoned(&self.connection, "sqlite billing connection")
            .map_err(|err| anyhow::anyhow!(err.to_string()))?;
        connection.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS billing_plans (
                plan_id TEXT NOT NULL PRIMARY KEY,
                plan_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS billing_subscriptions (
                org_id TEXT NOT NULL PRIMARY KEY,
                plan_id TEXT NOT NULL,
                status TEXT NOT NULL,
                version INTEGER NOT NULL,
                subscription_json TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS billing_invoices (
                org_id TEXT NOT NULL,
                period_key TEXT NOT NULL,
                status TEXT NOT NULL,
                invoice_json TEXT NOT NULL,
                PRIMARY KEY (org_id, period_key)
            );

            CREATE TABLE IF NOT EXISTS billing_adjustments (
                adjustment_id TEXT NOT NULL PRIMARY KEY,
                org_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                created_at TEXT NOT NULL,
                adjustment_json TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_billing_adjustments_org
                ON billing_adjustments (org_id, created_at, adjustment_id);

            CREATE TABLE IF NOT EXISTS billing_aether_settlements (
                settlement_id TEXT NOT NULL PRIMARY KEY,
                payment_hash TEXT NOT NULL UNIQUE,
                org_id TEXT NOT NULL,
                period_key TEXT NOT NULL,
                invoice_idempotency_key TEXT NOT NULL,
                settled_at TEXT NOT NULL,
                receipt_json TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_billing_aether_settlements_invoice
                ON billing_aether_settlements (org_id, period_key, settlement_id);

            CREATE TABLE IF NOT EXISTS billing_stripe_events (
                event_id TEXT NOT NULL PRIMARY KEY,
                object_id TEXT NOT NULL,
                created INTEGER NOT NULL,
                applied INTEGER NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_billing_stripe_events_object
                ON billing_stripe_events (object_id, applied, created);
            "#,
        )?;
        Ok(())
    }

    fn lock(&self) -> StoreResult<std::sync::MutexGuard<'_, Connection>> {
        beater_store::lock_poisoned(&self.connection, "sqlite billing connection")
    }
}

fn decode<T: serde::de::DeserializeOwned>(json: &str, what: &str) -> StoreResult<T> {
    serde_json::from_str(json).map_err(|err| StoreError::Integrity(format!("decode {what}: {err}")))
}

fn encode<T: serde::Serialize>(value: &T, what: &str) -> StoreResult<String> {
    serde_json::to_string(value).map_err(|err| StoreError::backend(format!("encode {what}: {err}")))
}

#[async_trait]
impl BillingStore for SqliteBillingStore {
    async fn put_plan(&self, plan: Plan) -> StoreResult<()> {
        let plan_json = encode(&plan, "plan")?;
        let connection = self.lock()?;
        connection
            .execute(
                "INSERT INTO billing_plans (plan_id, plan_json) VALUES (?1, ?2)
                 ON CONFLICT(plan_id) DO UPDATE SET plan_json = excluded.plan_json",
                params![plan.id.as_str(), plan_json],
            )
            .into_store_ctx("insert plan")?;
        Ok(())
    }

    async fn get_plan(&self, plan_id: &PlanId) -> StoreResult<Option<Plan>> {
        let connection = self.lock()?;
        let json: Option<String> = connection
            .query_row(
                "SELECT plan_json FROM billing_plans WHERE plan_id = ?1",
                params![plan_id.as_str()],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("get plan")?;
        match json {
            Some(json) => Ok(Some(decode(&json, "plan")?)),
            None => Ok(None),
        }
    }

    async fn list_plans(&self) -> StoreResult<Vec<Plan>> {
        let connection = self.lock()?;
        let mut statement = connection
            .prepare("SELECT plan_json FROM billing_plans ORDER BY plan_id ASC")
            .into_store_ctx("prepare list plans")?;
        let rows = statement
            .query_map([], |row| row.get::<_, String>(0))
            .into_store_ctx("query list plans")?;
        let mut plans = Vec::new();
        for row in rows {
            let json = row.into_store_ctx("read plan row")?;
            plans.push(decode(&json, "plan")?);
        }
        Ok(plans)
    }

    async fn create_subscription(&self, subscription: Subscription) -> StoreResult<Subscription> {
        let subscription_json = encode(&subscription, "subscription")?;
        let connection = self.lock()?;
        let changed = connection
            .execute(
                "INSERT OR IGNORE INTO billing_subscriptions
                   (org_id, plan_id, status, version, subscription_json)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    subscription.org_id.as_str(),
                    subscription.plan_id.as_str(),
                    subscription.status.as_str(),
                    subscription.version,
                    subscription_json
                ],
            )
            .into_store_ctx("insert subscription")?;
        if changed == 0 {
            return Err(StoreError::Conflict(format!(
                "subscription already exists for org {}",
                subscription.org_id.as_str()
            )));
        }
        Ok(subscription)
    }

    async fn get_subscription(&self, org_id: &OrganizationId) -> StoreResult<Option<Subscription>> {
        let connection = self.lock()?;
        let json: Option<String> = connection
            .query_row(
                "SELECT subscription_json FROM billing_subscriptions WHERE org_id = ?1",
                params![org_id.as_str()],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("get subscription")?;
        match json {
            Some(json) => Ok(Some(decode(&json, "subscription")?)),
            None => Ok(None),
        }
    }

    async fn change_subscription_plan(
        &self,
        org_id: &OrganizationId,
        expected_version: i64,
        new_plan_id: &PlanId,
        adjustments: &[BillingAdjustment],
    ) -> StoreResult<Subscription> {
        let mut connection = self.lock()?;
        let tx = connection
            .transaction()
            .into_store_ctx("begin change-plan tx")?;

        // Read the current row inside the tx for a consistent snapshot.
        let current_json: Option<String> = tx
            .query_row(
                "SELECT subscription_json FROM billing_subscriptions WHERE org_id = ?1",
                params![org_id.as_str()],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read subscription in tx")?;
        let current_json = current_json.ok_or_else(|| {
            StoreError::NotFound(format!("subscription for org {}", org_id.as_str()))
        })?;
        let mut subscription: Subscription = decode(&current_json, "subscription")?;

        // Optimistic update: only proceed if the version still matches.
        let next_version = expected_version
            .checked_add(1)
            .ok_or_else(|| StoreError::backend("subscription version overflow"))?;
        subscription.plan_id = new_plan_id.clone();
        subscription.version = next_version;
        let updated_json = encode(&subscription, "subscription")?;
        let changed = tx
            .execute(
                "UPDATE billing_subscriptions
                    SET plan_id = ?1, version = ?2, subscription_json = ?3
                  WHERE org_id = ?4 AND version = ?5",
                params![
                    new_plan_id.as_str(),
                    next_version,
                    updated_json,
                    org_id.as_str(),
                    expected_version
                ],
            )
            .into_store_ctx("optimistic subscription update")?;
        if changed == 0 {
            // Lost update: another writer bumped the version first. Roll back so
            // no adjustments leak.
            return Err(StoreError::Conflict(format!(
                "subscription for org {} was modified concurrently",
                org_id.as_str()
            )));
        }

        for adjustment in adjustments {
            let adjustment_json = encode(adjustment, "adjustment")?;
            tx.execute(
                "INSERT INTO billing_adjustments
                   (adjustment_id, org_id, kind, created_at, adjustment_json)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    adjustment.adjustment_id,
                    adjustment.org_id.as_str(),
                    adjustment.kind.as_str(),
                    adjustment.created_at.to_rfc3339(),
                    adjustment_json
                ],
            )
            .into_store_ctx("append proration adjustment")?;
        }

        tx.commit().into_store_ctx("commit change-plan tx")?;
        Ok(subscription)
    }

    async fn set_subscription_status(
        &self,
        org_id: &OrganizationId,
        status: SubscriptionStatus,
    ) -> StoreResult<Option<Subscription>> {
        let mut connection = self.lock()?;
        let tx = connection.transaction().into_store_ctx("begin status tx")?;
        let current_json: Option<String> = tx
            .query_row(
                "SELECT subscription_json FROM billing_subscriptions WHERE org_id = ?1",
                params![org_id.as_str()],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read subscription for status")?;
        let Some(current_json) = current_json else {
            return Ok(None);
        };
        let mut subscription: Subscription = decode(&current_json, "subscription")?;
        subscription.status = status;
        subscription.version = subscription
            .version
            .checked_add(1)
            .ok_or_else(|| StoreError::backend("subscription version overflow"))?;
        let updated_json = encode(&subscription, "subscription")?;
        tx.execute(
            "UPDATE billing_subscriptions
                SET status = ?1, version = ?2, subscription_json = ?3
              WHERE org_id = ?4",
            params![
                subscription.status.as_str(),
                subscription.version,
                updated_json,
                org_id.as_str()
            ],
        )
        .into_store_ctx("update subscription status")?;
        tx.commit().into_store_ctx("commit status tx")?;
        Ok(Some(subscription))
    }

    async fn insert_invoice_if_absent(&self, invoice: Invoice) -> StoreResult<Invoice> {
        let invoice_json = encode(&invoice, "invoice")?;
        let connection = self.lock()?;
        connection
            .execute(
                "INSERT OR IGNORE INTO billing_invoices
                   (org_id, period_key, status, invoice_json)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    invoice.org_id.as_str(),
                    invoice.period_key,
                    invoice.status.as_str(),
                    invoice_json
                ],
            )
            .into_store_ctx("insert invoice if absent")?;
        // Return whatever is stored (existing wins, so rollup is idempotent).
        let stored: String = connection
            .query_row(
                "SELECT invoice_json FROM billing_invoices WHERE org_id = ?1 AND period_key = ?2",
                params![invoice.org_id.as_str(), invoice.period_key],
                |row| row.get(0),
            )
            .into_store_ctx("read stored invoice")?;
        decode(&stored, "invoice")
    }

    async fn get_invoice(
        &self,
        org_id: &OrganizationId,
        period_key: &str,
    ) -> StoreResult<Option<Invoice>> {
        let connection = self.lock()?;
        let json: Option<String> = connection
            .query_row(
                "SELECT invoice_json FROM billing_invoices WHERE org_id = ?1 AND period_key = ?2",
                params![org_id.as_str(), period_key],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("get invoice")?;
        match json {
            Some(json) => Ok(Some(decode(&json, "invoice")?)),
            None => Ok(None),
        }
    }

    async fn list_invoices(&self, org_id: &OrganizationId) -> StoreResult<Vec<Invoice>> {
        let connection = self.lock()?;
        let mut statement = connection
            .prepare(
                "SELECT invoice_json FROM billing_invoices
                  WHERE org_id = ?1 ORDER BY period_key ASC",
            )
            .into_store_ctx("prepare list invoices")?;
        let rows = statement
            .query_map(params![org_id.as_str()], |row| row.get::<_, String>(0))
            .into_store_ctx("query list invoices")?;
        let mut invoices = Vec::new();
        for row in rows {
            let json = row.into_store_ctx("read invoice row")?;
            invoices.push(decode(&json, "invoice")?);
        }
        Ok(invoices)
    }

    async fn finalize_invoice(
        &self,
        org_id: &OrganizationId,
        period_key: &str,
    ) -> StoreResult<Invoice> {
        let mut connection = self.lock()?;
        let tx = connection
            .transaction()
            .into_store_ctx("begin finalize tx")?;
        let json: Option<String> = tx
            .query_row(
                "SELECT invoice_json FROM billing_invoices WHERE org_id = ?1 AND period_key = ?2",
                params![org_id.as_str(), period_key],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read invoice for finalize")?;
        let json = json.ok_or_else(|| {
            StoreError::NotFound(format!(
                "invoice for org {} period {period_key}",
                org_id.as_str()
            ))
        })?;
        let mut invoice: Invoice = decode(&json, "invoice")?;
        // Idempotent: only Draft transitions; any other status is left intact.
        if invoice.status == InvoiceStatus::Draft {
            invoice.status = InvoiceStatus::Finalized;
            let updated_json = encode(&invoice, "invoice")?;
            tx.execute(
                "UPDATE billing_invoices SET status = ?1, invoice_json = ?2
                  WHERE org_id = ?3 AND period_key = ?4 AND status = ?5",
                params![
                    invoice.status.as_str(),
                    updated_json,
                    org_id.as_str(),
                    period_key,
                    InvoiceStatus::Draft.as_str()
                ],
            )
            .into_store_ctx("finalize invoice")?;
        }
        tx.commit().into_store_ctx("commit finalize tx")?;
        Ok(invoice)
    }

    async fn set_invoice_status(
        &self,
        org_id: &OrganizationId,
        period_key: &str,
        status: InvoiceStatus,
    ) -> StoreResult<Invoice> {
        let mut connection = self.lock()?;
        let tx = connection
            .transaction()
            .into_store_ctx("begin invoice-status tx")?;
        let json: Option<String> = tx
            .query_row(
                "SELECT invoice_json FROM billing_invoices WHERE org_id = ?1 AND period_key = ?2",
                params![org_id.as_str(), period_key],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read invoice for status")?;
        let json = json.ok_or_else(|| {
            StoreError::NotFound(format!(
                "invoice for org {} period {period_key}",
                org_id.as_str()
            ))
        })?;
        let mut invoice: Invoice = decode(&json, "invoice")?;
        if invoice.status != status {
            invoice.status = status;
            let updated_json = encode(&invoice, "invoice")?;
            tx.execute(
                "UPDATE billing_invoices SET status = ?1, invoice_json = ?2
                  WHERE org_id = ?3 AND period_key = ?4",
                params![
                    invoice.status.as_str(),
                    updated_json,
                    org_id.as_str(),
                    period_key
                ],
            )
            .into_store_ctx("update invoice status")?;
        }
        tx.commit().into_store_ctx("commit invoice-status tx")?;
        Ok(invoice)
    }

    async fn append_adjustment(
        &self,
        adjustment: BillingAdjustment,
    ) -> StoreResult<BillingAdjustment> {
        let adjustment_json = encode(&adjustment, "adjustment")?;
        let connection = self.lock()?;
        connection
            .execute(
                "INSERT INTO billing_adjustments
                   (adjustment_id, org_id, kind, created_at, adjustment_json)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    adjustment.adjustment_id,
                    adjustment.org_id.as_str(),
                    adjustment.kind.as_str(),
                    adjustment.created_at.to_rfc3339(),
                    adjustment_json
                ],
            )
            .into_store_ctx("append adjustment")?;
        Ok(adjustment)
    }

    async fn list_adjustments(
        &self,
        org_id: &OrganizationId,
    ) -> StoreResult<Vec<BillingAdjustment>> {
        let connection = self.lock()?;
        let mut statement = connection
            .prepare(
                "SELECT adjustment_json FROM billing_adjustments
                  WHERE org_id = ?1 ORDER BY created_at ASC, adjustment_id ASC",
            )
            .into_store_ctx("prepare list adjustments")?;
        let rows = statement
            .query_map(params![org_id.as_str()], |row| row.get::<_, String>(0))
            .into_store_ctx("query list adjustments")?;
        let mut adjustments = Vec::new();
        for row in rows {
            let json = row.into_store_ctx("read adjustment row")?;
            adjustments.push(decode(&json, "adjustment")?);
        }
        Ok(adjustments)
    }

    async fn apply_aether_settlement(
        &self,
        receipt: &AetherSettlementReceipt,
        adjustment: &BillingAdjustment,
    ) -> StoreResult<bool> {
        let receipt_json = encode(receipt, "aether settlement receipt")?;
        let adjustment_json = encode(adjustment, "aether settlement adjustment")?;
        let mut connection = self.lock()?;
        let tx = connection
            .transaction()
            .into_store_ctx("begin aether settlement tx")?;

        let already_recorded: Option<String> = tx
            .query_row(
                "SELECT receipt_json FROM billing_aether_settlements
                  WHERE settlement_id = ?1 OR payment_hash = ?2",
                params![
                    receipt.settlement_id.as_str(),
                    receipt.payment_hash.as_str()
                ],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read aether settlement dedup")?;
        if let Some(existing_json) = already_recorded {
            let existing: AetherSettlementReceipt =
                decode(&existing_json, "aether settlement receipt")?;
            if &existing != receipt {
                return Err(StoreError::Conflict(format!(
                    "aether settlement duplicate key has different receipt binding: settlement_id {}, payment_hash {}",
                    receipt.settlement_id, receipt.payment_hash
                )));
            }
            tx.commit()
                .into_store_ctx("commit duplicate aether settlement tx")?;
            return Ok(false);
        }

        let invoice_json: Option<String> = tx
            .query_row(
                "SELECT invoice_json FROM billing_invoices WHERE org_id = ?1 AND period_key = ?2",
                params![receipt.org_id.as_str(), receipt.period_key.as_str()],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read invoice for aether settlement")?;
        let invoice_json = invoice_json.ok_or_else(|| {
            StoreError::NotFound(format!(
                "invoice for org {} period {}",
                receipt.org_id.as_str(),
                receipt.period_key
            ))
        })?;
        let mut invoice: Invoice = decode(&invoice_json, "invoice")?;
        validate_invoice_for_aether(receipt, &invoice)?;
        invoice.status = InvoiceStatus::Paid;
        let paid_invoice_json = encode(&invoice, "paid invoice")?;

        tx.execute(
            "INSERT INTO billing_aether_settlements
               (settlement_id, payment_hash, org_id, period_key, invoice_idempotency_key, settled_at, receipt_json)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                receipt.settlement_id.as_str(),
                receipt.payment_hash.as_str(),
                receipt.org_id.as_str(),
                receipt.period_key.as_str(),
                receipt.invoice_idempotency_key.as_str(),
                receipt.settled_at.to_rfc3339(),
                receipt_json,
            ],
        )
        .into_store_ctx("insert aether settlement")?;
        tx.execute(
            "UPDATE billing_invoices SET status = ?1, invoice_json = ?2
              WHERE org_id = ?3 AND period_key = ?4 AND status = ?5",
            params![
                InvoiceStatus::Paid.as_str(),
                paid_invoice_json,
                receipt.org_id.as_str(),
                receipt.period_key.as_str(),
                InvoiceStatus::Finalized.as_str(),
            ],
        )
        .into_store_ctx("mark invoice paid by aether")?;
        tx.execute(
            "INSERT INTO billing_adjustments
               (adjustment_id, org_id, kind, created_at, adjustment_json)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                adjustment.adjustment_id.as_str(),
                adjustment.org_id.as_str(),
                adjustment.kind.as_str(),
                adjustment.created_at.to_rfc3339(),
                adjustment_json,
            ],
        )
        .into_store_ctx("append aether settlement adjustment")?;

        tx.commit().into_store_ctx("commit aether settlement tx")?;
        Ok(true)
    }

    async fn record_stripe_event(
        &self,
        event_id: &str,
        object_id: &str,
        created: i64,
    ) -> StoreResult<bool> {
        let connection = self.lock()?;
        let changed = connection
            .execute(
                "INSERT OR IGNORE INTO billing_stripe_events
                   (event_id, object_id, created, applied)
                 VALUES (?1, ?2, ?3, 0)",
                params![event_id, object_id, created],
            )
            .into_store_ctx("record stripe event")?;
        Ok(changed == 1)
    }

    async fn last_applied_stripe_created(&self, object_id: &str) -> StoreResult<Option<i64>> {
        let connection = self.lock()?;
        let value: Option<i64> = connection
            .query_row(
                "SELECT MAX(created) FROM billing_stripe_events
                  WHERE object_id = ?1 AND applied = 1",
                params![object_id],
                |row| row.get(0),
            )
            .optional()
            .into_store_ctx("read last applied stripe event")?
            .flatten();
        Ok(value)
    }

    async fn mark_stripe_event_applied(&self, event_id: &str) -> StoreResult<()> {
        let connection = self.lock()?;
        connection
            .execute(
                "UPDATE billing_stripe_events SET applied = 1 WHERE event_id = ?1",
                params![event_id],
            )
            .into_store_ctx("mark stripe event applied")?;
        Ok(())
    }
}

/// Small extension to attach a static context string to a `rusqlite` error while
/// mapping it into a [`StoreError`].
trait IntoStoreCtx<T> {
    fn into_store_ctx(self, what: &str) -> StoreResult<T>;
}

impl<T> IntoStoreCtx<T> for Result<T, rusqlite::Error> {
    fn into_store_ctx(self, what: &str) -> StoreResult<T> {
        self.map_err(|err| StoreError::backend(format!("{what}: {err}")))
    }
}

// Re-exported so callers can build adjustment ids consistently if needed.
#[doc(hidden)]
pub fn adjustment_kind_str(kind: AdjustmentKind) -> &'static str {
    kind.as_str()
}

fn validate_invoice_for_aether(
    receipt: &AetherSettlementReceipt,
    invoice: &Invoice,
) -> StoreResult<()> {
    if invoice.status != InvoiceStatus::Finalized {
        return Err(StoreError::Conflict(format!(
            "aether settlement requires finalized invoice {}, got {}",
            invoice.idempotency_key,
            invoice.status.as_str()
        )));
    }
    if invoice.idempotency_key != receipt.invoice_idempotency_key {
        return Err(StoreError::Conflict(format!(
            "aether receipt invoice key {} does not match {}",
            receipt.invoice_idempotency_key, invoice.idempotency_key
        )));
    }
    if receipt.settled_value.currency != invoice.total.currency {
        return Err(StoreError::Conflict(format!(
            "aether settlement currency {} does not match invoice currency {}",
            receipt.settled_value.currency, invoice.total.currency
        )));
    }
    if receipt.settled_value.amount_micros < invoice.total.amount_micros {
        return Err(StoreError::Conflict(format!(
            "aether settled value {} is below invoice total {}",
            receipt.settled_value.amount_micros, invoice.total.amount_micros
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{InvoiceLineItem, PlanTier};
    use beater_core::Money;
    use chrono::Utc;
    use std::collections::BTreeMap;

    fn store() -> anyhow::Result<SqliteBillingStore> {
        SqliteBillingStore::in_memory()
    }

    fn sample_plan(id: &str) -> anyhow::Result<Plan> {
        Ok(Plan {
            id: PlanId::new(id).map_err(|e| anyhow::anyhow!(e.to_string()))?,
            tier: PlanTier::Pro,
            included: BTreeMap::new(),
            base_price: Money::usd_micros(10_000),
            overage_rates: BTreeMap::new(),
        })
    }

    fn sample_subscription(plan: &str) -> anyhow::Result<Subscription> {
        Ok(Subscription {
            org_id: OrganizationId::new("org-1").map_err(|e| anyhow::anyhow!(e.to_string()))?,
            plan_id: PlanId::new(plan).map_err(|e| anyhow::anyhow!(e.to_string()))?,
            status: SubscriptionStatus::Active,
            period_start: Utc::now(),
            period_end: Utc::now() + chrono::Duration::days(30),
            version: 1,
        })
    }

    #[tokio::test]
    async fn plan_roundtrip_and_upsert() -> anyhow::Result<()> {
        let store = store()?;
        store.put_plan(sample_plan("pro")?).await?;
        let fetched = store
            .get_plan(&PlanId::new("pro").map_err(|e| anyhow::anyhow!(e.to_string()))?)
            .await?;
        assert!(fetched.is_some());
        assert_eq!(store.list_plans().await?.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn create_subscription_is_unique() -> anyhow::Result<()> {
        let store = store()?;
        store.put_plan(sample_plan("pro")?).await?;
        store
            .create_subscription(sample_subscription("pro")?)
            .await?;
        let second = store.create_subscription(sample_subscription("pro")?).await;
        assert!(matches!(second, Err(StoreError::Conflict(_))));
        Ok(())
    }

    #[tokio::test]
    async fn insert_invoice_if_absent_is_idempotent() -> anyhow::Result<()> {
        let store = store()?;
        let org = OrganizationId::new("org-1").map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let invoice = Invoice {
            org_id: org.clone(),
            period_key: "2026-06".to_string(),
            line_items: vec![InvoiceLineItem {
                meter: None,
                description: "Base".to_string(),
                quantity: 0,
                included: 0,
                overage_quantity: 0,
                unit_rate: Money::usd_micros(10_000),
                amount: Money::usd_micros(10_000),
            }],
            total: Money::usd_micros(10_000),
            status: InvoiceStatus::Draft,
            idempotency_key: Invoice::idempotency_key_for(&org, "2026-06"),
            created_at: Utc::now(),
        };
        let first = store.insert_invoice_if_absent(invoice.clone()).await?;
        // A second insert with a *different* total must not overwrite the first.
        let mut tampered = invoice.clone();
        tampered.total = Money::usd_micros(999);
        let second = store.insert_invoice_if_absent(tampered).await?;
        assert_eq!(first.total, second.total);
        assert_eq!(store.list_invoices(&org).await?.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn stripe_event_dedup_and_ordering() -> anyhow::Result<()> {
        let store = store()?;
        // First delivery is recorded.
        assert!(store.record_stripe_event("evt_1", "sub_a", 100).await?);
        // Duplicate delivery is rejected.
        assert!(!store.record_stripe_event("evt_1", "sub_a", 100).await?);
        store.mark_stripe_event_applied("evt_1").await?;
        assert_eq!(store.last_applied_stripe_created("sub_a").await?, Some(100));
        Ok(())
    }
}
