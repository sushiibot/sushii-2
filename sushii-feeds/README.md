# sushii-feeds

Feeds service to send updates for VLive and RSS feeds. This runs separately from
sushii-2 and sends feed update messages to Discord via twilight-http-proxy.

```bash
RUST_LOG=info,sqlx=warn,sushii_feeds=debug
DATABASE_URL=
# make sure not to have http://
TWILIGHT_API_PROXY_URL=localhost:3001
```
