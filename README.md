# Beacon

## Development

The backend is written in Rust using [Axum](https://github.com/tokio-rs/axum).

```bash
cargo run
```

The server runs on `http://127.0.0.1:3000`. A health check is available at `/health`.
If you'd like to change the bind address, set `BEACON_ADDR` in your env-config.

```bash
BEACON_ADDR=127.0.0.1:4000 cargo run
```