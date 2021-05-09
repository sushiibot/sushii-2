# Generic Dockerfile which can build any sushii-2 packages
# docker build --build-arg TARGET=sushii-2 .

FROM rust:1.52 as build

# Which package to build
ARG TARGET

WORKDIR /usr/src/sushii
# Create a new empty shell project
RUN USER=root cargo new ${TARGET} --bin

# Copy over root manifests, Cargo.lock
COPY ./Cargo.lock ./Cargo.toml ./

# Copy local dependencies, this might be unused by some
COPY ./sushii-model ./sushii-model

# switch to target workspace project to run following commands in target dir
WORKDIR /usr/src/sushii/${TARGET}

# If there's a build.rs required in Cargo.toml
# This requires first file to always exist, 2nd one with * as optional
COPY ./${TARGET}/Cargo.toml ./${TARGET}/build.rs* ./

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source tree, migrations, queries, sqlx data
COPY ./${TARGET}/ ./

# Remove dummy compiled files (in workspace root)
RUN rm ../target/release/deps/*sushii*

# Test and build the actual package
RUN cargo test --release --locked
RUN cargo build --release --locked

## Final base image with only the built binary
FROM debian:buster-slim
ARG TARGET
ENV TARGET ${TARGET}

WORKDIR /config

# Fix sentry HTTPS calls with ca-certificates:
# https://github.com/getsentry/sentry-rust/issues/239
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Target dir is still in workspace root
COPY --from=build /usr/src/sushii/target/release/${TARGET} /usr/local/bin/${TARGET}

EXPOSE 9888
ENTRYPOINT "${TARGET}"
