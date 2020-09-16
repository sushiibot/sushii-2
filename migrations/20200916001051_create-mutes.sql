CREATE TABLE mutes (
    guild_id   BIGINT    NOT NULL,
    user_id    BIGINT    NOT NULL,
    start_time TIMESTAMP NOT NULL,
    end_time   TIMESTAMP,
    PRIMARY KEY (guild_id, user_id)
)
