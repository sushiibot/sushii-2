CREATE TABLE members (
    guild_id  BIGINT    NOT NULL,
    user_id   BIGINT    NOT NULL,
    join_time TIMESTAMP NOT NULL,
    PRIMARY KEY (guild_id, user_id)
)
