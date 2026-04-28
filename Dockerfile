FROM lukemathwalker/cargo-chef:0.1.77-rust-1.95 AS chef
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json ./
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian13@sha256:47b2d72ff90843eb8a768b5c2f89b40741843b639d065b9b937b07cd59b479c6 AS runtime

LABEL org.opencontainers.image.title="CitiesAPI"
LABEL org.opencontainers.image.description="Simple cities API written in Rust."
LABEL org.opencontainers.image.base.name="gcr.io/distroless/static-debian13"

USER nonroot:nonroot

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/citiesapi /usr/local/bin/citiesapi

EXPOSE 2022

ENTRYPOINT [ "/usr/local/bin/citiesapi" ]
