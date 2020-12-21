ALTER TABLE mutes
 ADD COLUMN pending BOOLEAN NOT NULL DEFAULT FALSE,
 ADD COLUMN case_id BIGINT,
 ADD CONSTRAINT fk_mod_action
    FOREIGN KEY (guild_id, case_id)
     REFERENCES mod_logs(guild_id, case_id);
