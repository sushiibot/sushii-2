-- Add migration script here
CREATE TABLE guild_configs (
    id                  BIGINT PRIMARY KEY,
    name                TEXT,
    prefix              TEXT,

    -- Join message text
    join_msg            TEXT,
    join_msg_enabled    BOOLEAN DEFAULT TRUE NOT NULL,

    -- Join message reaction
    join_react          TEXT,

    -- Leave message text
    leave_msg           TEXT,
    leave_msg_enabled   BOOLEAN DEFAULT TRUE NOT NULL,

    -- Join / leave messages channel
    msg_channel         BIGINT,

    -- Role assignments
    role_channel        BIGINT,
    role_config         JSONB,
    role_enabled        BOOLEAN DEFAULT TRUE NOT NULL,

    -- Auto delete invite links, default off
    invite_guard        BOOLEAN DEFAULT FALSE NOT NULL,

    -- Message deleted / edited log channel
    log_msg             BIGINT,
    log_msg_enabled     BOOLEAN DEFAULT TRUE NOT NULL,

    -- Moderation actions log channel
    log_mod             BIGINT,
    log_mod_enabled     BOOLEAN DEFAULT TRUE NOT NULL,

    -- Member join / leave log channel
    log_member          BIGINT,

    -- Mute role ID
    mute_role           BIGINT,
    -- Duration in seconds
    mute_duration       BIGINT,

    -- Should DM user on ban
    ban_dm_text         TEXT,
    ban_dm_enabled      BOOLEAN DEFAULT TRUE NOT NULL,

    -- Should DM user on kick
    kick_dm_text        TEXT,
    kick_dm_enabled     BOOLEAN DEFAULT TRUE NOT NULL,

    -- Should DM user on mute
    mute_dm_text        TEXT,
    mute_dm_enabled     BOOLEAN DEFAULT TRUE NOT NULL,

    -- Max number of unique mentions in a single message to auto mute
    max_mention         INTEGER
)
