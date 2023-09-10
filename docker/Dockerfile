FROM rust:1.72-buster AS builder
WORKDIR /usr/src
RUN apt-get update -y && apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new similarium-rs
WORKDIR /usr/src/similarium-rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY build.rs ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/similarium .
USER 1000
CMD ["./similarium"]
