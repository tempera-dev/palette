//! Aether settlement receipts for Beater billing.
//!
//! This module is deliberately chain-client-free. Aether verification belongs
//! at the wallet/indexer/SDK edge; Beater consumes a verified receipt and binds
//! it to an immutable invoice. That keeps the billing crate standalone while
//! still making Aether a first-class settlement rail.

use crate::BillingError;
use beater_core::{Money, OrganizationId, Timestamp};
use serde::{Deserialize, Serialize};

/// Result of attempting to apply a verified Aether settlement receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AetherSettlementApplication {
    Applied,
    Duplicate,
}

/// Verified Aether payment settlement bound to a Beater invoice.
///
/// `amount_minor_units` is a decimal string rather than a JSON number so token
/// quantities above JavaScript's safe integer range remain lossless across SDKs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AetherSettlementReceipt {
    /// Stable settlement identifier from the verifier/indexer.
    pub settlement_id: String,
    /// Aether payment envelope hash (`0x` + 64 lowercase hex chars).
    pub payment_hash: String,
    /// Chain/network namespace, e.g. `base-mainnet`, `solana-mainnet`.
    pub network: String,
    /// Chain id where one exists. Use 0 for non-EVM networks.
    pub chain_id: u64,
    /// Settled asset symbol or canonical asset id, e.g. `USDC`.
    pub asset: String,
    /// Lossless asset quantity in the asset's minor units.
    pub amount_minor_units: String,
    /// Decimal places used by `amount_minor_units`.
    pub asset_decimals: u8,
    /// Fiat value credited to the invoice, in Beater's integer money type.
    pub settled_value: Money,
    /// Invoice idempotency key this receipt pays.
    pub invoice_idempotency_key: String,
    /// Billing period key this receipt pays (`YYYY-MM`).
    pub period_key: String,
    /// Organization that owns the invoice.
    pub org_id: OrganizationId,
    /// Verifier identity that attests the Aether payment was settled.
    pub verifier: String,
    #[schema(value_type = String, format = DateTime)]
    pub settled_at: Timestamp,
}

impl AetherSettlementReceipt {
    pub fn validate(&self) -> Result<(), BillingError> {
        validate_non_empty("settlement_id", &self.settlement_id)?;
        validate_payment_hash(&self.payment_hash)?;
        validate_non_empty("network", &self.network)?;
        validate_non_empty("asset", &self.asset)?;
        validate_non_empty("invoice_idempotency_key", &self.invoice_idempotency_key)?;
        validate_non_empty("period_key", &self.period_key)?;
        validate_non_empty("verifier", &self.verifier)?;
        validate_minor_units(&self.amount_minor_units)?;
        if self.settled_value.amount_micros <= 0 {
            return Err(BillingError::InvalidSettlement(
                "settled value must be positive".to_string(),
            ));
        }
        Ok(())
    }
}

fn validate_non_empty(field: &str, value: &str) -> Result<(), BillingError> {
    if value.trim().is_empty() {
        return Err(BillingError::InvalidSettlement(format!(
            "{field} cannot be empty"
        )));
    }
    Ok(())
}

fn validate_minor_units(value: &str) -> Result<(), BillingError> {
    if value.is_empty() {
        return Err(BillingError::InvalidSettlement(
            "amount_minor_units cannot be empty".to_string(),
        ));
    }
    if value.len() > 1 && value.starts_with('0') {
        return Err(BillingError::InvalidSettlement(
            "amount_minor_units must be canonical decimal without leading zeros".to_string(),
        ));
    }
    if !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(BillingError::InvalidSettlement(
            "amount_minor_units must be decimal digits".to_string(),
        ));
    }
    if value == "0" {
        return Err(BillingError::InvalidSettlement(
            "amount_minor_units must be positive".to_string(),
        ));
    }
    Ok(())
}

fn validate_payment_hash(value: &str) -> Result<(), BillingError> {
    let Some(hex) = value.strip_prefix("0x") else {
        return Err(BillingError::InvalidSettlement(
            "payment_hash must start with 0x".to_string(),
        ));
    };
    if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(BillingError::InvalidSettlement(
            "payment_hash must be 32 bytes of hex".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use beater_core::OrganizationId;
    use chrono::Utc;

    fn receipt() -> Result<AetherSettlementReceipt, BillingError> {
        let org_id =
            OrganizationId::new("org-e2e").map_err(|err| BillingError::Backend(err.to_string()))?;
        Ok(AetherSettlementReceipt {
            settlement_id: "set_1".to_string(),
            payment_hash: "0x0831ce74c89358835be790d4a7794a2bb30cd7e5968bafb5cc99423ea5f25783"
                .to_string(),
            network: "base-mainnet".to_string(),
            chain_id: 8453,
            asset: "USDC".to_string(),
            amount_minor_units: "11000".to_string(),
            asset_decimals: 6,
            settled_value: Money::usd_micros(11_000),
            invoice_idempotency_key: "inv_org-e2e_2026-06".to_string(),
            period_key: "2026-06".to_string(),
            org_id,
            verifier: "aether-indexer:v1".to_string(),
            settled_at: Utc::now(),
        })
    }

    #[test]
    fn receipt_validation_accepts_canonical_values() -> Result<(), BillingError> {
        receipt()?.validate()
    }

    #[test]
    fn receipt_validation_rejects_lossy_or_unbound_values() -> Result<(), BillingError> {
        let mut value = receipt()?;
        value.amount_minor_units = "01".to_string();
        assert!(matches!(
            value.validate(),
            Err(BillingError::InvalidSettlement(_))
        ));

        let mut value = receipt()?;
        value.payment_hash = "0831ce74".to_string();
        assert!(matches!(
            value.validate(),
            Err(BillingError::InvalidSettlement(_))
        ));
        Ok(())
    }
}
