//! Rust `tracing` -> Palette example (R11.3).
//!
//! Bridges the idiomatic Rust `tracing` crate to Palette. The Palette Rust SDK
//! (`sdks/rust`, crate `palette`) is OpenTelemetry-native, so you can use either:
//!
//! 1. the SDK's ergonomic `palette::observe(...)` helper directly (shown here), or
//! 2. `tracing` + `tracing-opentelemetry` with an OTLP exporter pointed at
//!    paletted, in which case `tracing` spans become Palette spans automatically.
//!
//! Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! palette = { path = "../../sdks/rust" }
//! tracing = "0.1"
//! ```
//!
//! Run a local paletted (`docker compose up`) then `cargo run`.

use palette::{span_kind, PaletteConfig};

fn main() {
    palette::init(PaletteConfig {
        service_name: "palette-rust-tracing-example".to_string(),
        release_id: Some("tracing-example".to_string()),
        ..PaletteConfig::from_env()
    });

    // `observe` opens a Palette span with the right kind/sequence/release attrs.
    let decision = palette::observe("handle_refund", span_kind::AGENT_RUN, || {
        palette::set_input("late delivery refund after 31 days");
        let answer = palette::observe("call_model", span_kind::LLM_CALL, || {
            palette::set_output("Escalate: outside the refund window.");
            "escalate"
        });
        palette::set_output(answer);
        answer
    });

    println!("agent result: {decision}");
    palette::shutdown();
}
