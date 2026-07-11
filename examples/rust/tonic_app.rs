//! Tonic (gRPC) + Palette example (R11.3).
//!
//! A tonic gRPC service whose RPC handler emits a Palette agent trace per call
//! using the ergonomic Rust SDK. Demonstrates the first-class Rust gRPC adoption
//! path. The `.proto` and generated stubs are elided for brevity; the load-
//! bearing part is bracketing the RPC body in `palette::observe`.
//!
//! Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! palette = { path = "../../sdks/rust" }
//! tonic = "0.12"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! Run a local paletted (`docker compose up`) then `cargo run`.

use palette::{span_kind, PaletteConfig};

// In a real service these come from `tonic::include_proto!`. We model the RPC
// body so the Palette instrumentation pattern is clear and self-contained.
struct AgentRequest {
    prompt: String,
}

struct AgentReply {
    decision: String,
}

fn handle_rpc(req: AgentRequest) -> AgentReply {
    let decision = palette::observe("RunAgent", span_kind::AGENT_RUN, || {
        palette::set_input(req.prompt.clone());
        let answer = palette::observe("call_model", span_kind::LLM_CALL, || {
            palette::set_output("ok");
            "escalate"
        });
        palette::set_output(answer);
        answer.to_string()
    });
    AgentReply { decision }
}

#[tokio::main]
async fn main() {
    palette::init(PaletteConfig {
        service_name: "palette-rust-tonic-example".to_string(),
        release_id: Some("tonic-example".to_string()),
        ..PaletteConfig::from_env()
    });

    // Wire `handle_rpc` into your generated tonic service impl, e.g.
    //   async fn run_agent(&self, request: Request<AgentRequest>) -> ...
    //       { Ok(Response::new(handle_rpc(request.into_inner()))) }
    let reply = handle_rpc(AgentRequest {
        prompt: "refund please".to_string(),
    });
    println!("rpc decision: {}", reply.decision);
    palette::shutdown();
}
