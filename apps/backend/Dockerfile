# syntax=docker/dockerfile:1

FROM rust:1-bookworm AS builder
WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends cmake clang \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY apps/backend/Cargo.toml apps/backend/Cargo.toml
RUN mkdir -p apps/backend/src \
    && echo 'fn main() {}' > apps/backend/src/main.rs \
    && cargo build --release --bin beacon-backend \
    && rm -rf apps/backend/src

COPY apps/backend/src apps/backend/src
COPY apps/backend/migrations apps/backend/migrations
RUN touch apps/backend/src/main.rs \
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
