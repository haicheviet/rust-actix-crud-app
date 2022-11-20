FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY actix-api/ /app/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY actix-api/ /app/

RUN cargo build --release --bin actix-api

FROM debian:buster-slim AS runtime
WORKDIR /app
RUN apt-get update -y && apt-get install -y libpq-dev

COPY --from=builder /app/target/release/actix-api /usr/local/bin
CMD  ["/usr/local/bin/actix-api"]
