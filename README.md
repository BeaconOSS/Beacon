# Beacon

## Development

Requires [Rust](https://www.rust-lang.org/) and [Docker](https://www.docker.com/).

```bash
# start Postgres
docker compose up -d
# set DATABASE_URL
cp .env.example .env
# start the server
cargo run
```

The server runs on `http://127.0.0.1:3000`, with a health check at `/health`.