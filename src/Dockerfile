# 1st stage: Build the Rust app
FROM rust:1.74 as builder

WORKDIR /app

# Cache dependencies first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/projectf*

# Copy real source code
COPY . .

# Build for release
RUN cargo build --release

# 2nd stage: Tiny runtime container
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary
COPY --from=builder /app/target/release/projectf .

EXPOSE 8000

CMD ["./projectf"]
