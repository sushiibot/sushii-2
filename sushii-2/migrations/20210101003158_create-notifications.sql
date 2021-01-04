CREATE TABLE notifications (
    user_id  BIGINT NOT NULL,
    guild_id BIGINT NOT NULL,
    keyword  TEXT   NOT NULL,
    PRIMARY KEY (user_id, guild_id, keyword)
);

CREATE INDEX notification_keyword_idx ON notifications (keyword);
CREATE INDEX notification_guild_id_idx ON notifications (guild_id);
