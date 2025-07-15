FROM rust:1.78.0-slim-bookworm as builder
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bourso-api /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/bourso-api"]
