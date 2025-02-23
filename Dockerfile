# ---- Build Stage ----
FROM rust:1.85 AS builder

# Set working directory
WORKDIR /app

# Install required system dependencies (for Rust build)
RUN apt update && apt install -y lld clang -y

# Set environment variables
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

# Install SQLx CLI for migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

# Cache dependencies for efficient builds
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && cargo clean

# Copy source code and build the project
COPY . .
ARG DATABASE_URL
ENV SQLX_OFFLINE true
RUN cargo build --release

# ---- Runtime Stage ----
FROM ubuntu:22.04

# Install required system dependencies (for Rust runtime)
RUN apt-get update && apt-get install -y libpq-dev libc6 && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy compiled binary from builder stage
COPY --from=builder /app/target/release/sales_tracker /app/sales_tracker

# Copy migrations directory
COPY --from=builder /app/migrations /app/migrations

# Copy SQLx CLI
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Copy entrypoint script
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh

# Copy Sample Data
COPY --from=builder /app/sample_data.csv /app

RUN chmod +x /app/entrypoint.sh

ARG DATABASE_URL

# Expose the API port
EXPOSE 8080

# Command to run the API server
ENTRYPOINT ["/bin/bash", "-c", "/app/entrypoint.sh"]
