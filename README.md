![CI](https://github.com/sushiibot/sushii-2/workflows/CI/badge.svg)

# sushii-2

Rewrite of [sushii-bot](https://github.com/drklee3/sushii-bot) with async
[serenity-rs](https://github.com/serenity-rs/serenity/).

wip... again.

## Packages

sushii-2 is split up into a handful of packages, which uses a shared PostgreSQL
database.

* [`sushii-2`] - Discord bot process
* [`sushii-api`] - GraphQL API server used by [`sushii-web`]
* [`sushii-model`] - Shared models and SQL queries

[`sushii-2`]: ./sushii-2
[`sushii-api`]: ./sushii-api
[`sushii-model`]: ./sushii-model
[`sushii-web`]: https://github.com/sushiibot/sushii-web
