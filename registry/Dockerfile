# Build stage
FROM rust:1.74-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 wget && \
    rm -rf /var/lib/apt/lists/*

# Create directories for storage
RUN mkdir -p /var/lib/mdlibs/storage && \
    mkdir -p /data

# Copy the binary from builder
COPY --from=builder /app/target/release/mdlibs-registry /usr/local/bin/mdlibs-registry

# Copy migrations (if using embedded migrations)
# COPY migrations /app/migrations

# Create a non-root user
RUN useradd -r -u 1000 -U mdlibs && \
    chown -R mdlibs:mdlibs /var/lib/mdlibs /data

USER mdlibs

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

CMD ["mdlibs-registry"]
