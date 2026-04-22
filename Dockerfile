FROM lukemathwalker/cargo-chef:0.1.77-rust-1.95 AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml Cargo.lock src/ ./
RUN cargo build --release

FROM docker.io/library/debian:trixie-slim AS runtime

LABEL org.opencontainers.image.title="CitiesAPI"
LABEL org.opencontainers.image.description="Simple cities API written in Rust."
LABEL org.opencontainers.image.base.name="docker.io/library/debian:trixie-slim"

WORKDIR /app

COPY --from=builder /app/target/release/citiesapi /usr/local/bin

ENTRYPOINT ["/usr/local/bin/citiesapi"]
