# ============================================
# Stage 1: Build
# ============================================
FROM rust:1.83-bookworm AS builder

WORKDIR /app

# Dependency caching: copy only manifest files
COPY Cargo.toml Cargo.lock ./

# Dummy source for dependency pre-compilation
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source code and build
COPY . .

# Touch to trigger rebuild of our code (not dependencies)
RUN touch src/main.rs src/lib.rs

RUN cargo build --release

# ============================================
# Stage 2: Runtime
# ============================================
FROM debian:bookworm-slim

# rustls is used — no OpenSSL needed, only CA certificates
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Run as non-root user for security
RUN groupadd --system app && useradd --system --gid app app

# Copy binary
COPY --from=builder /app/target/release/url-shortener /usr/local/bin/

# Copy migrations for runtime auto-migration
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app

# Switch to non-root user
USER app

EXPOSE 3000

CMD ["url-shortener"]