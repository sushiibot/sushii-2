-- Add migration script here
CREATE TABLE mod_logs (
    guild_id    BIGINT    NOT NULL,
    case_id     BIGINT    NOT NULL,
    action      TEXT      NOT NULL,
    action_time TIMESTAMP NOT NULL,
    pending     BOOLEAN   NOT NULL,
    user_id     BIGINT    NOT NULL,
    user_tag    TEXT      NOT NULL,
    executor_id BIGINT,
    reason      TEXT,
    msg_id      BIGINT,
    PRIMARY KEY (guild_id, case_id)
)
