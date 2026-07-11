//! reqwest (outbound HTTP / LLM call) + Palette example (R11.3).
//!
//! Wraps an outbound `reqwest` HTTP call (e.g. to an LLM provider) in a Palette
//! `llm.call` span, recording the request as input and the response as output.
//! Demonstrates instrumenting client-side calls with the Rust SDK.
//!
//! Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! palette = { path = "../../sdks/rust" }
//! reqwest = { version = "0.12", features = ["json"] }
//! tokio = { version = "1", features = ["full"] }
//! serde_json = "1"
//! ```
//!
//! Run a local paletted (`docker compose up`) then `cargo run`.

use palette::{span_kind, PaletteConfig};

async fn call_llm(prompt: &str) -> String {
    // Bracket the outbound async call in an llm.call span via `observe_async`.
    palette::observe_async("call_model", span_kind::LLM_CALL, async move {
        palette::set_input(prompt.to_string());
        let client = reqwest::Client::new();
        let response = client
            .post("https://api.example-llm.test/v1/chat")
            .json(&serde_json::json!({ "prompt": prompt }))
            .send()
            .await;
        let text = match response {
            Ok(resp) => resp.text().await.unwrap_or_default(),
            // Network is intentionally unreachable in the example; record the
            // error as span output so the trace still shows the attempt.
            Err(err) => format!("(call failed: {err})"),
        };
        palette::set_output(text.clone());
        text
    })
    .await
}

#[tokio::main]
async fn main() {
    palette::init(PaletteConfig {
        service_name: "palette-rust-reqwest-example".to_string(),
        release_id: Some("reqwest-example".to_string()),
        ..PaletteConfig::from_env()
    });

    // Open the agent.run span, then run the instrumented async LLM call inside it.
    let prompt = "summarize the refund policy";
    let answer = palette::observe_async("handle_request", span_kind::AGENT_RUN, async {
        palette::set_input(prompt);
        let out = call_llm(prompt).await;
        palette::set_output(out.clone());
        out
    })
    .await;
    println!("llm answer: {answer}");
    palette::shutdown();
}
