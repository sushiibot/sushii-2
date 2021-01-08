CREATE TABLE reminders (
    user_id     BIGINT    NOT NULL,
    channel_id  BIGINT    NOT NULL,
    description TEXT      NOT NULL,
    set_at      TIMESTAMP NOT NULL,
    expire_at   TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, set_at)
)
