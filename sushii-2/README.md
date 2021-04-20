![CI](https://github.com/sushiibot/sushii-2/workflows/CI/badge.svg)
[![dependency status](https://deps.rs/repo/github/sushiibot/sushii-2/status.svg)](https://deps.rs/repo/github/sushiibot/sushii-2)

# sushii-2

Rewrite of [sushii-bot](https://github.com/drklee3/sushii-bot) with async
[serenity-rs](https://github.com/serenity-rs/serenity/).

wip... again.

## Running

Docker images are published on the [GitHub Container Registry](https://github.com/users/sushiibot/packages/container/package/sushii-2).

Images on the GitHub Packages Docker registry are no longer updated.

```bash
docker run --expose 9888 --env-file ./.env ghcr.io/sushiibot/sushii-2:latest
```

## Building

Alternatively, you can build from source with Rust

```bash
cargo build --release
```

## Configuration

Configuration options are set via environment variables read from an `.env`
file, options are as follows

```bash
RUST_LOG=info,sqlx=warn
DISCORD_TOKEN=
DEFAULT_PREFIX=
LASTFM_KEY=

# These options are for convenience to set the PostgreSQL options at the same time
POSTGRES_PASSWORD=
POSTGRES_USER=
POSTGRES_DB=

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}
SENTRY_DSN=

# sushii-image-server url
IMAGE_SERVER_URL=
# sushii-feeds url
FEED_SERVER_URL=
```
