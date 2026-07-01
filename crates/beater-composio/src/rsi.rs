//! The Composio ↔ RSI bridge: turn the live connector catalog into the
//! **policy-filtered** tool search space for the gated optimization loop.
//!
//! This closes the tool_set lever of π (§6.1) end-to-end:
//!
//! ```text
//! Composio catalog (ConnectorTool)
//!   → ConnectorToolPolicy.evaluate()          ← Beater-owned execution policy
//!   → beater_experiments::ToolCandidate       ← only tools that may execute
//!   → OptimizerStrategy::ConnectorToolSearch  ← deterministic ToolAdd proposals
//!   → run_optimization_round                  ← held-out Test gate + §21.4
//!                                               generalization-gap guardrail
//! ```
//!
//! Two invariants meet here and both are preserved:
//!
//! * **Policy before proposal.** A tool the connector policy would refuse to
//!   execute (denied, or high-risk without an explicit allowlist entry) is
//!   excluded from the search space *before* the proposer ever sees it — the
//!   optimizer cannot even suggest acquiring a capability the runtime forbids.
//!   Execution of an *accepted* tool is still re-checked by the same policy at
//!   invoke time, so a policy tightened after acceptance wins.
//! * **Proposal is not acceptance.** The candidates built here only enter the
//!   round as proposals; nothing ships unless it beats baseline on the held-out
//!   Test split with statistical confidence and survives the anti-overfitting
//!   guardrail.

use beater_experiments::ToolCandidate;

use crate::policy::{ConnectorToolPolicy, ConnectorToolPolicyDecision};
use crate::{skill, ConnectorTool};

/// Convert the connector catalog into the RSI tool search space, keeping only
/// tools the given policy allows to execute.
///
/// Each surviving tool carries its policy risk class (for the gate's audit
/// trail) and the complete `tools.json` entry
/// ([`skill::tool_definition_json`]) so an accepted candidate can be applied to
/// an agent repo without re-querying the provider. Ordering follows the input
/// catalog; the proposer re-ranks deterministically by failure-signal
/// relevance.
pub fn tool_candidates(
    tools: &[ConnectorTool],
    policy: &ConnectorToolPolicy,
) -> Vec<ToolCandidate> {
    tools
        .iter()
        .filter_map(|tool| {
            let decision: ConnectorToolPolicyDecision = policy.evaluate(tool);
            if !decision.allowed {
                return None;
            }
            Some(ToolCandidate {
                slug: tool.slug.clone(),
                name: tool.name.clone(),
                description: tool.description.clone(),
                tags: tool.tags.clone(),
                risk_class: decision.risk_class.as_str().to_string(),
                requires_connection: !tool.no_auth,
                definition: skill::tool_definition_json(tool),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn tool(slug: &str, description: &str) -> ConnectorTool {
        ConnectorTool {
            slug: slug.to_string(),
            name: slug.replace('_', " "),
            description: Some(description.to_string()),
            no_auth: false,
            toolkit: Some("github".to_string()),
            tags: vec![],
            input_schema: Some(json!({"type": "object"})),
        }
    }

    #[test]
    fn default_policy_admits_read_only_and_excludes_high_risk_tools() {
        let catalog = vec![
            tool("GITHUB_GET_REPOSITORY", "Get repository metadata"),
            tool("GITHUB_DELETE_REPOSITORY", "Delete a repository"),
            tool("GITHUB_CREATE_AN_ISSUE", "Open a new issue"),
        ];
        let candidates = tool_candidates(&catalog, &ConnectorToolPolicy::default());
        // Only the read-only tool survives the default deny-by-default policy —
        // the destructive and external-write tools never enter the RSI search
        // space at all.
        assert_eq!(candidates.len(), 1, "{candidates:?}");
        assert_eq!(candidates[0].slug, "GITHUB_GET_REPOSITORY");
        assert_eq!(candidates[0].risk_class, "read_only");
        assert!(candidates[0].requires_connection);
    }

    #[test]
    fn allowlist_admits_and_denylist_excludes() {
        let catalog = vec![
            tool("GITHUB_CREATE_AN_ISSUE", "Open a new issue"),
            tool("GITHUB_DELETE_REPOSITORY", "Delete a repository"),
        ];
        let policy = ConnectorToolPolicy::default()
            .with_allowed_tools(["GITHUB_CREATE_AN_ISSUE", "GITHUB_DELETE_REPOSITORY"])
            .with_denied_tools(["GITHUB_DELETE_REPOSITORY"]);
        let candidates = tool_candidates(&catalog, &policy);
        // The allowlisted write tool enters the search space; the denied tool is
        // excluded even though it is also allowlisted (deny wins).
        assert_eq!(candidates.len(), 1, "{candidates:?}");
        assert_eq!(candidates[0].slug, "GITHUB_CREATE_AN_ISSUE");
        assert_eq!(candidates[0].risk_class, "external_write");
    }

    #[test]
    fn candidates_carry_the_complete_tools_json_entry() {
        let catalog = vec![tool("GITHUB_GET_REPOSITORY", "Get repository metadata")];
        let candidates = tool_candidates(&catalog, &ConnectorToolPolicy::default());
        let definition = &candidates[0].definition;
        // The definition is the apply-ready tools.json entry, not a bare slug.
        assert_eq!(definition["name"], "GITHUB_GET_REPOSITORY");
        assert_eq!(definition["source"], "composio");
        assert!(definition["input_schema"].is_object());
        assert!(definition["skill_card"].is_string());
    }
}
