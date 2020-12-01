CREATE TABLE users (
    id           BIGINT    PRIMARY KEY,
    is_patron    BOOLEAN   NOT NULL,
    patron_emoji TEXT,
    rep          BIGINT    NOT NULL,
    fishies      BIGINT    NOT NULL,
    last_rep     TIMESTAMP,
    last_fishies TIMESTAMP,
    profile_data JSONB
)
