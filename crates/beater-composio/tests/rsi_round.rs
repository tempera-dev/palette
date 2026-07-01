//! Holistic RSI e2e: the Composio catalog, Beater's connector policy, the
//! tool_set proposer, and the deep statistical gate in ONE loop.
//!
//! ```text
//! catalog → ConnectorToolPolicy → rsi::tool_candidates → ConnectorToolSearch
//!         → run_optimization_round → held-out Test gate (paired t + CI)
//!                                  → §21.4 generalization-gap guardrail
//! ```
//!
//! Three properties are proven end-to-end:
//! 1. A policy-forbidden (destructive) tool never even enters the search space,
//!    even when its text is the *most* relevant to the failures.
//! 2. A relevant, policy-allowed tool that genuinely helps — uniformly across
//!    the optimization AND held-out splits — is proposed and accepted by the
//!    statistical gate, carrying the apply-ready `tools.json` definition.
//! 3. The same tool is REJECTED when its lift exists only on the split the
//!    optimizer could see: the held-out gate alone would pass it, but the
//!    generalization-gap guardrail catches the overfit. Tool acquisition gets
//!    no exemption from non-overfitting RSI.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use async_trait::async_trait;
use beater_composio::{rsi, ConnectorTool, ConnectorToolPolicy};
use beater_eval::{GateDecision, GatePolicy};
use beater_experiments::{
    run_optimization_round, CandidateChange, CandidateEvaluator, CaseScore, ChangeKind,
    FailureExample, OptimizationRoundConfig, OptimizerStrategy, Split,
};
use beater_judge::{
    GenerationRequest, GenerationResponse, JudgeProviderResult, ProviderCredentials, TextGenerator,
};
use serde_json::{json, Value};

fn tool(slug: &str, name: &str, description: &str) -> ConnectorTool {
    ConnectorTool {
        slug: slug.to_string(),
        name: name.to_string(),
        description: Some(description.to_string()),
        no_auth: false,
        toolkit: Some("github".to_string()),
        tags: vec![],
        input_schema: Some(json!({
            "type": "object",
            "required": ["owner", "repo"],
            "properties": {
                "owner": {"type": "string"},
                "repo": {"type": "string"}
            }
        })),
    }
}

/// The catalog a live `listConnectorTools` call would return. The destructive
/// tool's text is deliberately the closest match to the failure signal — if
/// policy filtering ever regressed, it would win the relevance ranking and the
/// assertions below would fail loudly.
fn catalog() -> Vec<ConnectorTool> {
    vec![
        tool(
            "GITHUB_GET_REPOSITORY",
            "Get a repository",
            "Get repository metadata such as the default branch.",
        ),
        tool(
            "GITHUB_DELETE_REPOSITORY",
            "Delete a repository",
            "Delete a repository and all its metadata, including the default branch.",
        ),
        tool(
            "CALENDAR_LIST_EVENTS",
            "List calendar events",
            "List upcoming calendar events.",
        ),
    ]
}

fn failures() -> Vec<FailureExample> {
    vec![FailureExample::from_parts(
        "What is the default branch of acme/widgets?",
        Some("main".to_string()),
        "I don't have access to that information",
        0.0,
        Some("missing repository metadata".to_string()),
    )]
}

/// ConnectorToolSearch is deterministic; a generator call would mean the round
/// took the wrong proposal path.
struct PanicGenerator;

#[async_trait]
impl TextGenerator for PanicGenerator {
    async fn generate(
        &self,
        _req: GenerationRequest,
        _credentials: ProviderCredentials,
    ) -> JudgeProviderResult<GenerationResponse> {
        panic!("ConnectorToolSearch must not consult the generation seam");
    }
}

/// Scripted paired scores: the injected evaluator stands in for actually
/// running the agent with/without the proposed tool over the case set.
struct ScriptedEvaluator {
    optimize: Vec<(f64, f64)>,
    test: Vec<(f64, f64)>,
}

#[async_trait]
impl CandidateEvaluator for ScriptedEvaluator {
    async fn evaluate(
        &self,
        candidate: &CandidateChange,
        _cases: &[Value],
    ) -> Result<Vec<CaseScore>, String> {
        // The round must only ever ask us to evaluate the policy-allowed tool.
        assert_eq!(candidate.kind, ChangeKind::ToolAdd);
        assert_eq!(candidate.target, "tool_set/GITHUB_GET_REPOSITORY");
        let mut out = Vec::new();
        for (i, (b, c)) in self.optimize.iter().enumerate() {
            let split = if i % 2 == 0 { Split::Train } else { Split::Val };
            out.push(CaseScore {
                split,
                baseline_score: *b,
                candidate_score: *c,
                covariate: None,
            });
        }
        for (b, c) in &self.test {
            out.push(CaseScore {
                split: Split::Test,
                baseline_score: *b,
                candidate_score: *c,
                covariate: None,
            });
        }
        Ok(out)
    }
}

fn round_config() -> OptimizationRoundConfig {
    let mut cfg = OptimizationRoundConfig::new(
        "answer repository metadata questions correctly",
        "You are a helpful assistant.",
        failures(),
        (0..12).map(|i| json!({ "case": i })).collect(),
        OptimizerStrategy::ConnectorToolSearch,
        GatePolicy {
            min_sample_size: 6,
            max_regression: 0.0,
            alpha: 0.05,
            comparison_count: 1,
        },
    );
    // The e2e seam under test: live catalog → Beater policy → search space.
    cfg.available_tools = rsi::tool_candidates(&catalog(), &ConnectorToolPolicy::default());
    cfg
}

/// A tool that genuinely helps — uniform lift on the optimization AND held-out
/// splits — is proposed from the policy-filtered catalog and ACCEPTED by the
/// held-out statistical gate. The destructive tool never appears anywhere.
#[tokio::test]
async fn round_accepts_a_policy_allowed_tool_that_generalizes() {
    let cfg = round_config();
    // Policy pre-filter: both read-only tools enter the search space; the
    // destructive tool is excluded even though its text is the most relevant
    // to the failures.
    let slugs: Vec<&str> = cfg
        .available_tools
        .iter()
        .map(|t| t.slug.as_str())
        .collect();
    assert_eq!(
        slugs,
        vec!["GITHUB_GET_REPOSITORY", "CALENDAR_LIST_EVENTS"],
        "policy admits read-only tools only"
    );

    let evaluator = ScriptedEvaluator {
        optimize: vec![(0.5, 0.9); 6],
        test: vec![(0.5, 0.9); 6],
    };
    let outcome = run_optimization_round(
        cfg,
        &PanicGenerator,
        ProviderCredentials::new("openai", "sk-test"),
        &evaluator,
    )
    .await
    .unwrap_or_else(|err| panic!("{err}"));

    // Exactly one candidate: the relevant, policy-allowed read-only tool. The
    // irrelevant calendar tool was dropped by relevance, the destructive tool
    // by policy.
    assert_eq!(outcome.evaluated.len(), 1);
    let eval = &outcome.evaluated[0];
    assert_eq!(eval.gate.decision, GateDecision::Pass);
    assert!(!eval.overfit.overfit);
    assert!(eval.accepted);

    let accepted = outcome
        .accepted
        .unwrap_or_else(|| panic!("expected the generalizing tool to be accepted"));
    assert_eq!(accepted.kind, ChangeKind::ToolAdd);
    assert_eq!(accepted.target, "tool_set/GITHUB_GET_REPOSITORY");
    assert_eq!(accepted.proposed_by, OptimizerStrategy::ConnectorToolSearch);
    // The audit trail names the policy risk class and the gating invariant.
    assert!(accepted.rationale.contains("read_only"));
    assert!(accepted.rationale.contains("held-out Test gate"));
}

/// The SAME tool is rejected when its lift exists only on the optimization
/// split: the held-out Test gate alone sees "no regression" and would pass it,
/// but the §21.4 generalization-gap guardrail flags the overfit. Tool
/// acquisition is subject to exactly the same anti-overfitting statistics as
/// every other π lever.
#[tokio::test]
async fn round_rejects_a_tool_whose_lift_does_not_generalize() {
    let evaluator = ScriptedEvaluator {
        optimize: vec![(0.5, 0.9); 6], // big lift where the optimizer can see
        test: vec![(0.5, 0.5); 6],     // nothing on the held-out split
    };
    let outcome = run_optimization_round(
        round_config(),
        &PanicGenerator,
        ProviderCredentials::new("openai", "sk-test"),
        &evaluator,
    )
    .await
    .unwrap_or_else(|err| panic!("{err}"));

    assert_eq!(outcome.evaluated.len(), 1);
    let eval = &outcome.evaluated[0];
    assert!(
        eval.overfit.overfit,
        "gap guardrail must flag optimize-only lift: {:?}",
        eval.overfit
    );
    assert!(!eval.accepted);
    assert!(
        outcome.accepted.is_none(),
        "an overfit tool must never be applied"
    );
}

/// Tightening the policy to deny the remaining tool empties the search space,
/// and the round surfaces a typed error instead of silently proposing nothing —
/// a misconfigured tool round is loud, not a fake-clean no-op.
#[tokio::test]
async fn round_errors_when_policy_empties_the_search_space() {
    let mut cfg = round_config();
    cfg.available_tools = rsi::tool_candidates(
        &catalog(),
        &ConnectorToolPolicy::default()
            .with_denied_tools(["GITHUB_GET_REPOSITORY", "CALENDAR_LIST_EVENTS"]),
    );
    assert!(cfg.available_tools.is_empty());

    let evaluator = ScriptedEvaluator {
        optimize: vec![(0.5, 0.9); 6],
        test: vec![(0.5, 0.9); 6],
    };
    let err = run_optimization_round(
        cfg,
        &PanicGenerator,
        ProviderCredentials::new("openai", "sk-test"),
        &evaluator,
    )
    .await
    .err()
    .unwrap_or_else(|| panic!("expected an error on an empty search space"));
    assert!(
        err.to_string().contains("policy-filtered tool catalog"),
        "{err}"
    );
}
