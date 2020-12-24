CREATE TABLE cached_users (
    id            BIGINT    PRIMARY KEY,
    avatar_url    TEXT      NOT NULL,
    name          TEXT      NOT NULL,
    discriminator INTEGER   NOT NULL,
    -- Seen user, updated once per day
    last_checked  TIMESTAMP NOT NULL
)
