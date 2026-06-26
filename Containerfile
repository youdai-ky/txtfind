# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.96.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ------------------------------
# Stage 2. Build for runtime
# ------------------------------
FROM debian:bookworm-slim

WORKDIR /work

COPY --from=builder /app/target/release/txtfind /usr/local/bin/txtfind

ENTRYPOINT ["txtfind"]