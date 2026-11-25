# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM rust:1-bookworm AS builder

WORKDIR /usr/src/app

# 1. Cache dependency build layer
COPY Cargo.toml Cargo.lock ./
COPY migration/Cargo.toml ./migration/Cargo.toml

RUN mkdir -p src migration/src \
    && echo "fn main() {}" > src/main.rs \
    && echo "pub struct Migrator;" > migration/src/lib.rs \
    && cargo build --release --bin todo-app-v2 \
    && rm -rf src migration/src

# 2. Copy full source and build release binary
COPY src ./src
COPY migration ./migration

RUN touch src/main.rs migration/src/lib.rs \
    && cargo build --release --bin todo-app-v2

# =============================================================================
# Stage 2: Production Runtime
# =============================================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user and application directories
RUN useradd -m -u 10001 appuser \
    && mkdir -p /app/uploads \
    && chown -R appuser:appuser /app

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/todo-app-v2 /app/todo-app-v2

USER appuser

ENV APP_HOST=0.0.0.0 \
    APP_PORT=8080 \
    RUST_LOG=info

EXPOSE 8080

HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -fsS http://127.0.0.1:8080/api/v1/health/ready || exit 1

CMD ["/app/todo-app-v2"]
