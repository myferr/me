FROM rust:latest AS backend-builder

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y musl-tools pkg-config curl

RUN rustup target add aarch64-unknown-linux-musl

COPY . .

ENV OPENSSL_STATIC=1
ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN cargo build --release -p backend --target aarch64-unknown-linux-musl \
 && strip target/aarch64-unknown-linux-musl/release/backend || true

FROM alpine:latest

WORKDIR /usr/src/app

RUN apk add --no-cache ca-certificates curl

COPY --from=backend-builder /usr/src/app/target/aarch64-unknown-linux-musl/release/backend /usr/src/app/backend

COPY ./content /usr/src/app/content

EXPOSE 2025

CMD ["/bin/sh", "-c", "/usr/src/app/backend"]
