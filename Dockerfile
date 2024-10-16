## Base Image with dependencies + Cargo Chef
FROM rust:1.81-slim-bullseye AS base
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --version ^0.1

## Cache stage using Cargo Chef to generate recipe
FROM base AS cache
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

## Builder base stage
FROM base AS builder
WORKDIR /app
COPY --from=cache /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

## Build Debug Mode
FROM builder AS build-debug
WORKDIR /app
COPY . .
RUN cargo build
CMD ["/app/target/debug/discord-ani-schedule"]

## Build Production Mode
FROM builder AS build-prod
WORKDIR /app
COPY . .
RUN RUSTFLAGS="-C link-arg=-s" cargo build --locked --no-default-features --release

## Release Stage
FROM gcr.io/distroless/cc-debian12 AS release
COPY --from=build-prod /app/target/release/discord-ani-schedule /
CMD ["./discord-ani-schedule"]
