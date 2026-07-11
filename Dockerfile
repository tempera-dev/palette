# syntax=docker/dockerfile:1.7

FROM rust:1-bookworm AS rust-base
WORKDIR /app
RUN apt-get update \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
    clang \
    curl \
    libssl-dev \
    pkg-config \
    protobuf-compiler \
  && rm -rf /var/lib/apt/lists/*

FROM rust-base AS chef
RUN cargo install cargo-chef --locked

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust-deps
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust-deps AS paletted-builder
COPY . .
RUN cargo build --release --locked -p paletted

FROM rust-deps AS palettectl-builder
COPY . .
RUN cargo build --release --locked -p palettectl

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/* \
  && useradd --create-home --uid 10001 palette \
  && mkdir -p /data \
  && chown -R palette:palette /data
COPY --from=paletted-builder /app/target/release/paletted /usr/local/bin/paletted
USER palette
WORKDIR /data
EXPOSE 8080 4317
ENV PALETTE_TRACE_WRITE_DRAIN_INTERVAL_MS=250
ENV PALETTE_TRACE_INGESTED_DRAIN_INTERVAL_MS=250
HEALTHCHECK --interval=10s --timeout=3s --start-period=10s --retries=12 \
  CMD curl -fsS http://127.0.0.1:8080/health || exit 1
ENTRYPOINT ["paletted"]
CMD ["--addr", "0.0.0.0:8080", "--otlp-grpc-addr", "0.0.0.0:4317", "--data-dir", "/data"]

FROM runtime AS tools
USER root
COPY --from=palettectl-builder /app/target/release/palettectl /usr/local/bin/palettectl
USER palette
