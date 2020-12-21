CREATE TABLE user_levels (
    user_id      BIGINT    NOT NULL,
    guild_id     BIGINT    NOT NULL,
    msg_all_time BIGINT    NOT NULL,
    msg_month    BIGINT    NOT NULL,
    msg_week     BIGINT    NOT NULL,
    msg_day      BIGINT    NOT NULL,
    last_msg     TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, guild_id)
)
