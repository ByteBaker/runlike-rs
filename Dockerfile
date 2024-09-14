FROM rust:latest AS builder

# Alipne uses musl instead of glibc
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache docker-cli

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/runlike /usr/local/bin/runlike

ENTRYPOINT ["/usr/local/bin/runlike"]
