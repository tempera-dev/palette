//! Eval harness for browser agents — the loop that turns observation into
//! improvement.
//!
//! It runs a browser agent's planned steps against a [`BrowserDriver`], collects
//! [`StepTriple`]s, scores them with the browser evaluators in `beater-eval`,
//! and A/B-tests a baseline vs a candidate variant (a different prompt or agent
//! code) through the regression gate. Because each run produces the same
//! `trace.browser_steps` shape, the comparison is attributable to the variant,
//! not to a drifting page.
//!
//! [`BrowserAgentAdapter`] implements the existing
//! [`beater_experiments::AgentAdapter`], so a browser agent plugs directly into
//! `run_agent_experiment` / `evaluate_gate` like any other agent.

use beater_browser::{BrowserAction, BrowserDriver, BrowserError, LlmDecision, StepTriple};
use beater_browser_capture::browser_trace;
use beater_datasets::DatasetCase;
use beater_eval::{
    compare_paired_scores, evaluate_deterministic, EvalError, EvaluationCase, EvaluatorKind,
    EvaluatorSpec, GateDecision, GatePolicy,
};
use beater_experiments::{AgentAdapter, AgentAdapterError, AgentRunOutput, HarnessContext};
use beater_schema::EvaluatorLane;
use serde_json::json;
use std::marker::PhantomData;

/// Errors raised by the harness.
#[derive(Debug, thiserror::Error)]
pub enum HarnessError {
    #[error("browser driver error: {0}")]
    Driver(#[from] BrowserError),
    #[error("evaluator error: {0}")]
    Eval(#[from] EvalError),
    #[error("trace projection error: {0}")]
    Capture(String),
    #[error("no tasks to compare")]
    Empty,
}

/// A single planned step: the (optional) decision that produced it and the
/// action to execute. The decision carries the prompt, so it is captured for
/// iteration/replay.
pub type PlannedStep = (Option<LlmDecision>, BrowserAction);

/// A browser agent variant: given a task goal, produce the steps it would take.
/// This is what differs between a baseline and a candidate (prompt or code).
pub trait BrowserAgent {
    fn label(&self) -> &str;
    fn plan(&self, goal: &str) -> Vec<PlannedStep>;
}

/// Run a planned scenario against a driver, collecting the step triples.
pub async fn run_scenario<D: BrowserDriver>(
    driver: &mut D,
    start_url: &str,
    steps: Vec<PlannedStep>,
) -> Result<Vec<StepTriple>, HarnessError> {
    let mut observation_before = driver.goto(start_url).await?;
    let mut triples = Vec::with_capacity(steps.len());
    for (index, (decision, action)) in steps.into_iter().enumerate() {
        let outcome = driver.act(&action).await?;
        let observation_after = outcome.observation.clone();
        triples.push(StepTriple {
            seq: index as u64,
            observation_before,
            decision,
            action,
            outcome,
        });
        observation_before = observation_after;
    }
    Ok(triples)
}

/// Score a run's step triples with a deterministic browser evaluator.
pub fn score_run(triples: &[StepTriple], kind: EvaluatorKind) -> Result<f64, HarnessError> {
    let trace = browser_trace(triples).map_err(|err| HarnessError::Capture(err.to_string()))?;
    let spec = EvaluatorSpec {
        id: kind.catalog_id().to_string(),
        lane: EvaluatorLane::DeterministicWasi,
        kind,
    };
    let case = EvaluationCase {
        input: json!(null),
        output: json!(null),
        reference: None,
        trace: Some(trace),
    };
    Ok(evaluate_deterministic(&spec, &case)?.score)
}

/// Outcome of an A/B run: per-variant scores and the regression-gate decision.
#[derive(Clone, Debug, PartialEq)]
pub struct AbResult {
    pub baseline_label: String,
    pub candidate_label: String,
    pub baseline_scores: Vec<f64>,
    pub candidate_scores: Vec<f64>,
    pub delta: f64,
    pub decision: GateDecision,
}

/// Run baseline and candidate agents over the same tasks against fresh drivers,
/// score each run with `kind`, and gate the paired comparison. `make_driver`
/// yields a fresh driver per run (a seeded `MockDriver` for deterministic A/B,
/// or a real backend for live runs).
#[allow(clippy::too_many_arguments)]
pub async fn run_ab<D, F>(
    tasks: &[String],
    start_url: &str,
    baseline: &dyn BrowserAgent,
    candidate: &dyn BrowserAgent,
    kind: EvaluatorKind,
    policy: &GatePolicy,
    mut make_driver: F,
) -> Result<AbResult, HarnessError>
where
    D: BrowserDriver,
    F: FnMut() -> D,
{
    if tasks.is_empty() {
        return Err(HarnessError::Empty);
    }
    let mut baseline_scores = Vec::with_capacity(tasks.len());
    let mut candidate_scores = Vec::with_capacity(tasks.len());
    for task in tasks {
        let mut driver = make_driver();
        let triples = run_scenario(&mut driver, start_url, baseline.plan(task)).await?;
        baseline_scores.push(score_run(&triples, kind.clone())?);

        let mut driver = make_driver();
        let triples = run_scenario(&mut driver, start_url, candidate.plan(task)).await?;
        candidate_scores.push(score_run(&triples, kind.clone())?);
    }
    let comparison = compare_paired_scores(&baseline_scores, &candidate_scores, policy)?;
    Ok(AbResult {
        baseline_label: baseline.label().to_string(),
        candidate_label: candidate.label().to_string(),
        baseline_scores,
        candidate_scores,
        delta: comparison.delta,
        decision: comparison.decision,
    })
}

/// Adapts a [`BrowserAgent`] + driver factory to the experiment harness'
/// [`AgentAdapter`], so browser agents run through `run_agent_experiment` and
/// `evaluate_gate` exactly like any other agent. The task goal is read from the
/// dataset case input; the returned trace carries `browser_steps` for scoring.
pub struct BrowserAgentAdapter<A, D, F> {
    agent: A,
    start_url: String,
    make_driver: F,
    _driver: PhantomData<fn() -> D>,
}

impl<A, D, F> BrowserAgentAdapter<A, D, F>
where
    A: BrowserAgent + Send + Sync,
    D: BrowserDriver,
    F: Fn() -> D + Send + Sync,
{
    pub fn new(agent: A, start_url: impl Into<String>, make_driver: F) -> Self {
        Self {
            agent,
            start_url: start_url.into(),
            make_driver,
            _driver: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<A, D, F> AgentAdapter for BrowserAgentAdapter<A, D, F>
where
    A: BrowserAgent + Send + Sync,
    D: BrowserDriver,
    F: Fn() -> D + Send + Sync,
{
    async fn run_case(
        &self,
        case: DatasetCase,
        _context: HarnessContext,
    ) -> Result<AgentRunOutput, AgentAdapterError> {
        let goal = case.input.as_str().unwrap_or_default();
        let mut driver = (self.make_driver)();
        let triples = run_scenario(&mut driver, &self.start_url, self.agent.plan(goal))
            .await
            .map_err(AgentAdapterError::backend)?;
        let trace = browser_trace(&triples).map_err(AgentAdapterError::backend)?;
        let grounded = triples
            .iter()
            .filter(|triple| triple.outcome.grounding.matched_element)
            .count();
        let output = json!({ "steps": triples.len(), "grounded": grounded });
        Ok(AgentRunOutput {
            output,
            trace: Some(trace),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beater_browser::{BrowserEngine, MockDriver, FIXTURE_KNOWN_SELECTOR};

    /// A variant that clicks the correct (grounded) element for every task.
    struct GroundedAgent;
    impl BrowserAgent for GroundedAgent {
        fn label(&self) -> &str {
            "grounded-prompt"
        }
        fn plan(&self, goal: &str) -> Vec<PlannedStep> {
            vec![(
                Some(decision(goal, FIXTURE_KNOWN_SELECTOR)),
                BrowserAction::Click {
                    selector: FIXTURE_KNOWN_SELECTOR.to_string(),
                },
            )]
        }
    }

    /// A regressed variant whose prompt targets a selector that does not exist.
    struct RegressedAgent;
    impl BrowserAgent for RegressedAgent {
        fn label(&self) -> &str {
            "regressed-prompt"
        }
        fn plan(&self, goal: &str) -> Vec<PlannedStep> {
            vec![(
                Some(decision(goal, "#hallucinated")),
                BrowserAction::Click {
                    selector: "#hallucinated".to_string(),
                },
            )]
        }
    }

    fn decision(goal: &str, selector: &str) -> LlmDecision {
        LlmDecision {
            model: Some("claude".to_string()),
            prompt: json!({ "goal": goal }),
            output: json!({ "action": "click", "selector": selector }),
            reasoning: Some(format!("click {selector} to accomplish: {goal}")),
        }
    }

    fn tasks(n: usize) -> Vec<String> {
        (0..n).map(|i| format!("complete checkout {i}")).collect()
    }

    fn driver() -> MockDriver {
        MockDriver::with_conformance_fixture()
    }

    #[tokio::test]
    async fn regressed_prompt_is_gated_as_regression() {
        // Full loop: run baseline + candidate over the same frozen page, score
        // grounding, and gate. The regressed prompt must FAIL the gate.
        let result = run_ab(
            &tasks(12),
            "https://shop.example/cart",
            &GroundedAgent,
            &RegressedAgent,
            EvaluatorKind::BrowserGrounding { min_ratio: 1.0 },
            &GatePolicy::default(),
            driver,
        )
        .await
        .unwrap_or_else(|err| panic!("{err}"));

        assert!(
            result.baseline_scores.iter().all(|s| *s == 1.0),
            "grounded prompt should resolve every selector"
        );
        assert!(
            result.candidate_scores.iter().all(|s| *s == 0.0),
            "regressed prompt should miss every selector"
        );
        assert!(result.delta < 0.0, "candidate regressed grounding");
        assert_eq!(
            result.decision,
            GateDecision::FailRegression,
            "a prompt that breaks grounding must fail the gate"
        );
    }

    #[tokio::test]
    async fn equivalent_prompt_does_not_regress() {
        // Positive control: the same prompt vs itself must not be gated as a
        // regression (otherwise the gate would block healthy changes).
        let result = run_ab(
            &tasks(12),
            "https://shop.example/cart",
            &GroundedAgent,
            &GroundedAgent,
            EvaluatorKind::BrowserGrounding { min_ratio: 1.0 },
            &GatePolicy::default(),
            driver,
        )
        .await
        .unwrap_or_else(|err| panic!("{err}"));
        assert_ne!(result.decision, GateDecision::FailRegression);
    }

    #[test]
    fn engine_label_is_stable() {
        assert_eq!(
            MockDriver::new(BrowserEngine::Webkit).engine().as_str(),
            "webkit"
        );
    }
}
