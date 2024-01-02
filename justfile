# Run lint and test
all: lint test

# Run `cargo build`
build:
  cargo build

# Run the webserver with auto reload on file changes
watch:
  cargo watch -x run
  
# Run the server
server:
  cargo run

# Run the server in release mode
server_release:
  cargo run --release

# Run SQLX migrations
migrate:
  sqlx migrate run 

# Run all tests
test:
  cargo test

# Lint the project with fmt and clippy
lint: fmt clippy

fmt:
  cargo fmt --all -- --check

clippy:
  cargo clippy -- -D warnings

# Start postgres (with vector support) with dev defaults
postgres:
  docker run \
    --name similarium_postgres \
    -e POSTGRES_PASSWORD=s3cr3t \
    -e POSTGRES_USER=similarium \
    -e POSTGRES_DB=similarium \
    -p 127.0.0.1:5432:5432 \
    -d \
    tensorchord/pgvecto-rs:pg15-v0.1.13
