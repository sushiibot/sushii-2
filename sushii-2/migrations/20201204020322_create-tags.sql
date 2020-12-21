CREATE TABLE tags (
    owner_id  BIGINT    NOT NULL,
    guild_id  BIGINT    NOT NULL,
    tag_name  TEXT      NOT NULL,
    content   TEXT      NOT NULL,
    use_count BIGINT    NOT NULL,
    created   TIMESTAMP NOT NULL,
    PRIMARY KEY (guild_id, tag_name)
)
