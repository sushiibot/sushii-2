## Compile sushii
FROM rust:1.48 as build

# create a new empty shell project
WORKDIR /usr/src/sushii
RUN USER=root cargo new sushii-api --bin

# copy over manifests, Cargo.lock is in workspace root
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./sushii-api/Cargo.toml ./sushii-api/Cargo.toml

# copy local dependencies
COPY ./sushii-model ./sushii-model

# switch to sushii-api workspace project to run following commands in sushii-api dir
WORKDIR /usr/src/sushii/sushii-api

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree, migrations, queries, sqlx data
COPY ./sushii-api/src ./src
COPY ./sushii-api/sqlx-data.json ./sqlx-data.json

# build for release, remove dummy compiled files (in workspace root)
RUN rm ../target/release/deps/*sushii_api*

RUN cargo test --release --locked
RUN cargo build --release --locked

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config

# Fix sentry HTTPS calls with ca-certificates:
# https://github.com/getsentry/sentry-rust/issues/239
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# target dir is still in workspace root
COPY --from=build /usr/src/sushii/target/release/sushii-api /usr/local/bin/sushii-api

EXPOSE 8080
ENTRYPOINT ["sushii-api"]
