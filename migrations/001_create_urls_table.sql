CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE urls (
    id              UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    short_code      VARCHAR(16)     UNIQUE NOT NULL,
    original_url    TEXT            NOT NULL,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    expires_at      TIMESTAMPTZ,
    click_count     BIGINT          NOT NULL DEFAULT 0,
    last_clicked_at TIMESTAMPTZ,
    is_active       BOOLEAN         NOT NULL DEFAULT TRUE
);

-- Listeleme ve sıralama için
CREATE INDEX idx_urls_created_at ON urls (created_at);

-- Süresi dolmuşları temizleme sorgusu için (partial index: sadece NULL olmayan satırlar)
CREATE INDEX idx_urls_expires_at ON urls (expires_at)
    WHERE expires_at IS NOT NULL;
