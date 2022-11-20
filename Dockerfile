FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
RUN apt-get update -y && apt-get install -y libssl-dev clang llvm-dev libclang-dev

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin actix-api

FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/actix-api /app
CMD ["/app/actix-api"]
