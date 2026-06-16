# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake clang \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml backend/Cargo.toml
RUN mkdir backend/src \
    && echo 'fn main() {}' > backend/src/main.rs \
    && cargo build --release --bin beacon-backend \
    && rm -rf backend/src

COPY backend/src backend/src
COPY backend/migrations backend/migrations
RUN touch backend/src/main.rs \
    && cargo build --release --bin beacon-backend

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --no-create-home beacon
WORKDIR /app
COPY --from=builder /app/target/release/beacon-backend /usr/local/bin/beacon-backend

USER beacon
ENV BEACON_ADDR=0.0.0.0:3000
EXPOSE 3000
CMD ["beacon-backend"]
