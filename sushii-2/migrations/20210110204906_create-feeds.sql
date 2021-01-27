CREATE TABLE feeds (
    feed_id  TEXT PRIMARY KEY,
    metadata JSONB
);

CREATE TABLE feed_subscriptions (
    feed_id      TEXT   NOT NULL,
    guild_id     BIGINT NOT NULL,
    channel_id   BIGINT NOT NULL,
    mention_role BIGINT,
    PRIMARY KEY (feed_id, channel_id),
    CONSTRAINT fk_feed_subscription_feed_id
        FOREIGN KEY(feed_id) 
            REFERENCES feeds(feed_id)
            ON DELETE CASCADE
);

CREATE TABLE feed_items (
    feed_id TEXT NOT NULL,
    item_id TEXT NOT NULL,
    PRIMARY KEY (feed_id, item_id),
    CONSTRAINT fk_feed_item_feed_id
        FOREIGN KEY(feed_id) 
            REFERENCES feeds(feed_id)
            ON DELETE CASCADE
);
