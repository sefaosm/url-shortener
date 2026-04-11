CREATE TABLE click_events (
    id          UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    url_id      UUID            NOT NULL REFERENCES urls(id) ON DELETE CASCADE,
    clicked_at  TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    ip_address  INET,
    user_agent  TEXT,
    referer     TEXT
);

-- Belirli URL'nin tıklamalarını çekmek için
CREATE INDEX idx_click_events_url_id ON click_events (url_id);

-- Zaman bazlı analitik sorguları için
CREATE INDEX idx_click_events_clicked_at ON click_events (clicked_at);
