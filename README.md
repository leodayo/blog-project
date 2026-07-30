# Blog Project

A full‑stack blog system built with Rust, featuring:
- HTTP (actix‑web) and gRPC (tonic) APIs
- JWT authentication with password hashing (Argon2)
- PostgreSQL with sqlx (migrations included)
- CLI client (HTTP and gRPC)
- WASM frontend (Leptos)

## Project Structure
```
- blog-proto/         shared protobuf definitions (generated code)
- blog-server/        backend server (HTTP + gRPC)
- blog-client/        client library (HTTP and gRPC)
- blog-cli/           CLI client (two binaries: http and grpc)
- blog-wasm/          WASM frontend (Leptos)
```

## Quick Start

### Prerequisites

- Rust (1.75+)
- PostgreSQL (local or Docker)
- wasm-pack (for WASM build)

### Environment Variables

Create a .env file (see the example .env_example) or set the variables in the environment.


### Run Backend Server
```bash
cargo run -p blog-server
```

The server starts:
- HTTP API on http://localhost:8080
- gRPC API on http://localhost:50051

### Run CLI Client

HTTP client:
```bash
cargo run -p blog-cli --bin blog-cli-http -- register --username alice --email alice@example.com --password secret
cargo run -p blog-cli --bin blog-cli-http -- login --username alice --password secret
cargo run -p blog-cli --bin blog-cli-http -- create --title "Hello" --content "World"
cargo run -p blog-cli --bin blog-cli-http -- list --limit 5 --offset 0
```

gRPC client:
```bash
cargo run -p blog-cli --bin blog-cli-grpc -- --server http://localhost:50051 register --username bob --email bob@example.com --password secret
```

### Build and Run WASM Frontend

1. Build the WASM module:
```bash
cd blog-wasm
wasm-pack build --target web
```

2. Serve the static files (e.g., with miniserve or Python):
```bash
miniserve --index index.html . --port 8000
```
or
```bash
python3 -m http.server 8000
```

3. Open http://localhost:8000 in a browser.

Make sure the backend server is running (CORS is configured via env variables).

## CLI Commands (HTTP & gRPC)

| Command | Description |
|---------|-------------|
| register \-\-username \<u\> \-\-email \<e\> \-\-password \<p\> | Create a new user |
| login \-\-username \<u\> \-\-password \<p\> | Log in and save token |
| create \-\-title \<t\> \-\-content \<c\> | Create a new post (auth required) |
| get \-\-id \<id\> | Get a single post |
| update \-\-id \<id\> \-\-title \<t\> \-\-content \<c\> | Update a post (auth required, author only) |
| delete \-\-id \<id\> | Delete a post (auth required, author only) |
| list \-\-limit \<n\> \-\-offset \<o\> | List posts with pagination |

## Future Improvements
- Testing: add comprehensive unit and integration tests, refactor repositories to use Executor for better test isolation.
- Authentication: implement refresh tokens (stored in HTTP‑only cookies) and short‑lived JWT tokens in memory.
- Docker: provide Docker images and docker-compose for easy deployment.
- HTTPS: support Let's Encrypt / certbot for secure connections.
- Frontend: add pagination, better error handling, and visual polish.
- CI/CD: set up GitHub Actions for automated builds and tests.
