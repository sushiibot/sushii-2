# sushii-2

Rewrite of [sushii-bot](https://github.com/drklee3/sushii-bot) with async
[serenity-rs](https://github.com/serenity-rs/serenity/).

wip... again.

## Features

Slimmed down feature set of original sushii-bot with a focus on moderation tools

-   [ ] moderation tools
    -   [x] ban / unban
    -   [x] kick
    -   [x] prune
    -   [x] warn
    -   [x] mute / unmute
        -   [x] timed mutes
            -   [x] set duration per mute (s!!mute [ids] [duration and reason])
            -   [x] remove time duration (indefinite)
            -   [x] adjust existing mute duration
        -   [ ] allow muting users who left
        -   [ ] mute users who joined in last x minutes
    -   [x] cases
        -   [x] reason
        -   [x] history
    -   [ ] user ID lookup (list of ids to check if in guild, banned, muted, etc)
    -   [x] roles
        -   [x] json support
        -   [x] toml support
    -   [ ] settings
        -   [ ] dm users on action with reason (mutes only)
            -   [ ] toggle
            -   [ ] set message
        -   [ ] default mute duration (0 to disable)
        -   [ ] default num messages delete messages on ban
    -   [ ] auto boost role?
-   user
    -   avatar
    -   userinfo
    -   notifications

## Running

Docker images are published on the [GitHub Container Registry](https://github.com/users/drklee3/packages/container/package/sushii-2).

Images on the GitHub Packages Docker registry are no longer updated.

```bash
docker run --expose 9888 --env-file ./.env ghcr.io/drklee3/sushii-2:latest
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
OWNER_IDS=

# These options are for convenience to set the PostgreSQL options at the same time
POSTGRES_PASSWORD=
POSTGRES_USER=
POSTGRES_DB=

DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:5432/${POSTGRES_DB}
```
