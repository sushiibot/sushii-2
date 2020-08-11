## Compile actix-web server
FROM rust:1.45 as build

# create a new empty shell project
RUN USER=root cargo new --bin sources
WORKDIR /sources

# copy over manifests
COPY ./Cargo.lock ./Cargo.toml ./
# cargo is a dir and COPY copies dir contents
COPY ./cargo ./cargo

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree, test files, migrations, etc
COPY ./src ./tests ./

# build for release, remove dummy compiled files
RUN rm ./target/release/deps/*sushii-2*

RUN cargo test --release
RUN cargo build --release

## Final base image with only the picatch binary
FROM debian:buster-slim
COPY --from=back /sources/target/release/sushii-2 /sushii-2

ENTRYPOINT ["/sushii-2"]
