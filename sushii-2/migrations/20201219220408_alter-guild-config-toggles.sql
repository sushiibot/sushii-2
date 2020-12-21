ALTER TABLE guild_configs
 ADD COLUMN log_member_enabled BOOLEAN DEFAULT TRUE NOT NULL,
 ADD COLUMN warn_dm_text       TEXT,
 ADD COLUMN warn_dm_enabled    BOOLEAN DEFAULT TRUE NOT NULL,
DROP COLUMN ban_dm_text,
DROP COLUMN ban_dm_enabled,
DROP COLUMN kick_dm_text,
DROP COLUMN kick_dm_enabled
