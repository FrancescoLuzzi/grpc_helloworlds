FROM rust:1.80-alpine3.20 as chef
WORKDIR /app
RUN apk add protoc musl-dev --no-cache && \
    cargo install cargo-chef

FROM chef as planner
COPY rust_proto .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY rust_proto .
COPY proto/greeter.proto .
# Build our project
RUN cargo build --release --features dockerproto

FROM alpine:3.20 AS runtime
WORKDIR /app
RUN apk add --no-cache openssl ca-certificates
COPY --from=builder /app/target/release/rust_proto rust_proto
ENTRYPOINT ["./rust_proto"]

