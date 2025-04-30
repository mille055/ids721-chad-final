# 1st stage: Build the Rust app
FROM rust:latest AS builder

WORKDIR /app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY data ./data
COPY static ./static

RUN cargo fetch
RUN cargo build --release

# 2nd stage: Tiny runtime container
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/projectf .
COPY data ./data
COPY Cargo.toml Cargo.lock ./
COPY static ./static

EXPOSE 8000

CMD ["./projectf"]
