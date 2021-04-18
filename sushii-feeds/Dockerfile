## Compile sushii
FROM rust:1.48 as build

RUN rustup component add rustfmt

# create a new empty shell project
WORKDIR /usr/src/sushii
RUN USER=root cargo new sushii-feeds --bin

# copy over manifests, Cargo.lock is in workspace root
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./sushii-feeds/Cargo.toml ./sushii-feeds/Cargo.toml

# copy local dependencies
COPY ./sushii-model ./sushii-model

# switch to sushii-feeds workspace project to run following commands in sushii-feeds dir
WORKDIR /usr/src/sushii/sushii-feeds

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree, proto, feeds, etc
COPY ./sushii-feeds/src ./src
COPY ./sushii-feeds/feeds.json ./feeds.json

# build for release, remove dummy compiled files (in workspace root)
RUN rm ../target/release/deps/*sushii_feeds*

RUN cargo test --release --locked
RUN cargo build --release --locked

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config

# Fix sentry HTTPS calls with ca-certificates:
# https://github.com/getsentry/sentry-rust/issues/239
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# target dir is still in workspace root
COPY --from=build /usr/src/sushii/target/release/sushii-feeds /usr/local/bin/sushii-feeds

ENTRYPOINT ["sushii-feeds"]
