-- Add migration script here
CREATE TABLE IF NOT EXISTS guild_configs (
    id                BIGINT PRIMARY KEY,
    name              TEXT,
    prefix            TEXT,
    join_msg          TEXT,
    join_react        TEXT,
    leave_msg         TEXT,
    msg_channel       BIGINT,
    role_channel      BIGINT,
    role_config       JSONB,
    invite_guard      BOOLEAN,
    log_msg           BIGINT,
    log_mod           BIGINT,
    log_member        BIGINT,
    mute_role         BIGINT,
    max_mention       INTEGER,
    disabled_channels BIGINT[]
)
