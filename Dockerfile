# ============================================
# Stage 1: Build
# ============================================
FROM rust:1.83-bookworm AS builder

WORKDIR /app

# Dependency caching: sadece manifest dosyalarını kopyala
COPY Cargo.toml Cargo.lock ./

# Dummy source oluştur → bağımlılıklar compile edilsin
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs
RUN cargo build --release
RUN rm -rf src

# Gerçek kaynak kodu kopyala ve build et
COPY . .

# Timestamp'leri güncelle, cargo rebuild tetiklensin
RUN touch src/main.rs src/lib.rs

# SQLx compile-time check: DB olmadan build için offline mode
ENV SQLX_OFFLINE=true

RUN cargo build --release

# ============================================
# Stage 2: Runtime
# ============================================
FROM debian:bookworm-slim

# rustls kullandığımız için OpenSSL gerekmez, sadece CA sertifikaları
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Binary'yi kopyala
COPY --from=builder /app/target/release/url-shortener /usr/local/bin/

# Migration dosyalarını da kopyala (runtime'da migrate için)
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app

EXPOSE 3000

CMD ["url-shortener"]
