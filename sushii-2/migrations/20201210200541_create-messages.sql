CREATE TABLE messages (
    message_id   BIGINT    PRIMARY KEY,
    author_id    BIGINT    NOT NULL,
    channel_id   BIGINT    NOT NULL,
    guild_id     BIGINT    NOT NULL,
    created      TIMESTAMP NOT NULL,
    content      TEXT      NOT NULL,
    msg          JSONB     NOT NULL
)
