## Compile actix-web server
FROM rust:1.45 as build

# create a new empty shell project
WORKDIR /usr/src/sushii
RUN USER=root cargo init --bin

# copy over manifests
COPY ./Cargo.lock ./Cargo.toml ./
# cargo is a dir and COPY copies dir contents
COPY ./.cargo ./.cargo

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree, migrations, sqlx data
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./sqlx-data.json ./sqlx-data.json

# build for release, remove dummy compiled files
RUN rm ./target/release/deps/*sushii_2*

RUN cargo test --release
RUN cargo build --release

## Final base image with only the picatch binary
FROM debian:buster-slim

WORKDIR /config
RUN apt-get update && apt-get install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /usr/src/sushii/target/release/sushii-2 /usr/local/bin/sushii-2

EXPOSE 9888
ENTRYPOINT ["sushii-2"]
