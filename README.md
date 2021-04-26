![CI](https://github.com/sushiibot/sushii-2/workflows/CI/badge.svg)

# sushii-2

Rewrite of [sushii-bot](https://github.com/drklee3/sushii-bot) with async
[serenity-rs](https://github.com/serenity-rs/serenity/).

wip... again.

## Packages

sushii-2 is split up into a handful of packages which uses a shared PostgreSQL
database and send Discord API requests through twilight-http-proxy.

* [`sushii-2`] - Main Discord bot process
* [`sushii-api`] - GraphQL API server used by [`sushii-web`] (deprecated)
* [`sushii-feeds`] - Feed service for RSS and vlive.tv feeds
* [`sushii-model`] - Shared models and SQL queries
* [`sushii-rules`] - Experimental rules engine
* [`sushii-webhooks`] - Web server to handle external webhook services

## Docker Images

Docker images can be built for each package with the following, with the `xxxx`
replaced with the package name.

```bash
docker build --build-arg TARGET=sushii-xxxx .

# For example sushii-2
docker build --build-arg TARGET=sushii-2 .
# Rules
docker build --build-arg TARGET=sushii-rules .
```


[`sushii-2`]: ./sushii-2
[`sushii-api`]: ./sushii-api
[`sushii-feeds`]: ./sushii-feeds
[`sushii-model`]: ./sushii-model
[`sushii-rules`]: ./sushii-rules
[`sushii-webhooks`]: ./sushii-webhooks

[`sushii-web`]: https://github.com/sushiibot/sushii-web
