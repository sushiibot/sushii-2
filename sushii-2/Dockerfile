## Compile sushii
FROM rust:1.52 as build

RUN rustup component add rustfmt

# create a new empty shell project
WORKDIR /usr/src/sushii
RUN USER=root cargo new sushii-2 --bin

# copy over manifests, Cargo.lock is in workspace root
COPY ./Cargo.lock ./Cargo.toml ./
COPY ./sushii-2/Cargo.toml ./sushii-2/Cargo.toml

# copy local dependencies
COPY ./sushii-model ./sushii-model

# switch to sushii-2 workspace project to run following commands in sushii-2 dir
WORKDIR /usr/src/sushii/sushii-2

# build.rs required since in Cargo.toml
COPY ./sushii-2/build.rs ./build.rs

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree, migrations, queries, sqlx data
COPY ./sushii-2/src ./src
COPY ./sushii-2/migrations ./migrations
COPY ./sushii-2/sqlx-data.json ./sqlx-data.json

# build for release, remove dummy compiled files (in workspace root)
RUN rm ../target/release/deps/*sushii_2*

RUN cargo test --release --locked
RUN cargo build --release --locked

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config

# Fix sentry HTTPS calls with ca-certificates:
# https://github.com/getsentry/sentry-rust/issues/239
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# target dir is still in workspace root
COPY --from=build /usr/src/sushii/target/release/sushii-2 /usr/local/bin/sushii-2

EXPOSE 9888
ENTRYPOINT ["sushii-2"]
