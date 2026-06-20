use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

pub type Timestamp = DateTime<Utc>;

pub trait Clock: Send + Sync {
    fn now(&self) -> Timestamp;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Timestamp {
        DateTime::<Utc>::from(SystemTime::now())
    }
}

#[derive(Clone, Debug)]
pub struct FixedClock {
    now: Timestamp,
}

impl FixedClock {
    pub fn new(now: Timestamp) -> Self {
        Self { now }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> Timestamp {
        self.now
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum IdError {
    #[error("identifier cannot be empty")]
    Empty,
    #[error("identifier contains whitespace: {0}")]
    Whitespace(String),
}

macro_rules! id_type {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                let value = value.into();
                if value.is_empty() {
                    return Err(IdError::Empty);
                }
                if value.chars().any(char::is_whitespace) {
                    return Err(IdError::Whitespace(value));
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = IdError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

id_type!(TenantId);
id_type!(OrganizationId);
id_type!(ProjectId);
id_type!(EnvironmentId);
id_type!(AgentId);
id_type!(AgentReleaseId);
id_type!(RunId);
id_type!(TraceId);
id_type!(SpanId);
id_type!(ArtifactId);
id_type!(DatasetId);
id_type!(DatasetVersionId);
id_type!(DatasetCaseId);
id_type!(ExperimentId);
id_type!(ExperimentRunId);
id_type!(EvaluatorId);
id_type!(EvaluatorVersionId);
id_type!(EvalResultId);
id_type!(GateId);
id_type!(GateRunId);
id_type!(ReviewQueueId);
id_type!(ReviewTaskId);
id_type!(AnnotationId);
id_type!(CalibrationReportId);
id_type!(PromptId);
id_type!(PromptVersionId);
id_type!(ApiKeyId);
id_type!(ProviderSecretId);
id_type!(JudgeCallId);
id_type!(UsageRecordId);
id_type!(AuditEventId);
id_type!(WebhookEndpointId);
id_type!(IdempotencyKey);
id_type!(Sha256Hash);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenantScope {
    pub tenant_id: TenantId,
    pub project_id: ProjectId,
    pub environment_id: EnvironmentId,
}

impl TenantScope {
    pub fn new(tenant_id: TenantId, project_id: ProjectId, environment_id: EnvironmentId) -> Self {
        Self {
            tenant_id,
            project_id,
            environment_id,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Usd => "USD",
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub enum MoneyError {
    #[error("currency mismatch: {left} != {right}")]
    CurrencyMismatch { left: Currency, right: Currency },
    #[error("money amount overflow")]
    Overflow,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount_micros: i64,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount_micros: i64, currency: Currency) -> Self {
        Self {
            amount_micros,
            currency,
        }
    }

    pub fn usd_micros(amount_micros: i64) -> Self {
        Self::new(amount_micros, Currency::Usd)
    }

    pub fn try_add(&self, other: &Self) -> Result<Self, MoneyError> {
        self.ensure_same_currency(other)?;
        let amount_micros = self
            .amount_micros
            .checked_add(other.amount_micros)
            .ok_or(MoneyError::Overflow)?;
        Ok(Self::new(amount_micros, self.currency))
    }

    pub fn try_sub(&self, other: &Self) -> Result<Self, MoneyError> {
        self.ensure_same_currency(other)?;
        let amount_micros = self
            .amount_micros
            .checked_sub(other.amount_micros)
            .ok_or(MoneyError::Overflow)?;
        Ok(Self::new(amount_micros, self.currency))
    }

    fn ensure_same_currency(&self, other: &Self) -> Result<(), MoneyError> {
        if self.currency == other.currency {
            Ok(())
        } else {
            Err(MoneyError::CurrencyMismatch {
                left: self.currency,
                right: other.currency,
            })
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenCounts {
    pub input: u64,
    pub output: u64,
    pub reasoning: u64,
    pub cache_read: u64,
}

impl TokenCounts {
    pub fn total(&self) -> u64 {
        self.input + self.output + self.reasoning
    }
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PageRequest {
    pub limit: u32,
    pub cursor: Option<String>,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            limit: 100,
            cursor: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, next_cursor: Option<String>) -> Self {
        Self { items, next_cursor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_reject_empty_and_whitespace() {
        assert_eq!(TenantId::new(""), Err(IdError::Empty));
        assert!(matches!(
            TenantId::new("tenant one"),
            Err(IdError::Whitespace(value)) if value == "tenant one"
        ));
        assert_eq!(
            TenantId::new("tenant-one").map(|id| id.to_string()),
            Ok("tenant-one".to_string())
        );
    }

    #[test]
    fn token_total_excludes_cache_read() {
        let counts = TokenCounts {
            input: 10,
            output: 20,
            reasoning: 5,
            cache_read: 100,
        };
        assert_eq!(counts.total(), 35);
    }

    #[test]
    fn money_math_requires_matching_currency() {
        let left = Money::usd_micros(100);
        let right = Money::usd_micros(40);

        assert_eq!(left.try_sub(&right), Ok(Money::usd_micros(60)));
        assert_eq!(right.try_add(&left), Ok(Money::usd_micros(140)));
        assert_eq!(
            serde_json::to_value(Money::usd_micros(1)).unwrap_or_else(|err| panic!("{err}"))
                ["currency"],
            "USD"
        );
    }
}
