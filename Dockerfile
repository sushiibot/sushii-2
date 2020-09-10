## Compile actix-web server
FROM rust:1.45 as build

# create a new empty shell project
RUN USER=root cargo new --bin sources
WORKDIR /sources

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
COPY --from=build /sources/target/release/sushii-2 ./sushii-2

EXPOSE 9888
ENTRYPOINT ["/config/sushii-2"]
