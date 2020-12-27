CREATE TABLE cached_guilds (
    id            BIGINT PRIMARY KEY,
    name          TEXT   NOT NULL,
    member_count  BIGINT NOT NULL,
    icon_url      TEXT   NOT NULL,
    -- Array but converted to comma sep string
    features      TEXT   NOT NULL,
    splash_url    TEXT,
    banner_url    TEXT
)
