//! Five-line Palette quickstart: instrument an agent and emit a trace.
//!
//! Run a local paletted (e.g. `docker compose up`), then:
//!
//! ```sh
//! PALETTE_TENANT_ID=demo PALETTE_PROJECT_ID=demo PALETTE_ENVIRONMENT_ID=local \
//!     cargo run --example quickstart
//! ```
//!
//! It emits an agent.run -> agent.plan -> llm.call trace. Open the dashboard and
//! click the trace.

use palette::{span_kind, PaletteConfig};

fn main() {
    palette::init(PaletteConfig {
        service_name: "palette-rust-quickstart".to_string(),
        release_id: Some("quickstart".to_string()),
        ..PaletteConfig::from_env()
    });

    let result = palette::observe("handle_refund", span_kind::AGENT_RUN, || {
        palette::set_input("late delivery refund after 31 days");

        let plan = palette::observe("make_plan", span_kind::AGENT_PLAN, || {
            "look up refund policy".to_string()
        });

        let decision = palette::observe("call_model", span_kind::LLM_CALL, || {
            palette::set_input(plan.clone());
            palette::set_output("Escalate: order is outside the standard refund window.");
            "escalate"
        });

        palette::set_output(decision);
        decision
    });

    println!("agent result: {result}");
    palette::shutdown();
    println!("trace flushed -- open the dashboard to inspect it");
}
