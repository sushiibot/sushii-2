# sushii-model

SQL models shared between sushii-2 and sushii-api

**Note:** Since `sqlx-data.json` is required and this crate has feature gated
queries, `cargo make sqlx-prepare` should be used instead of `cargo sqlx
prepare` to merge the two `sqlx-data.json` files together.
