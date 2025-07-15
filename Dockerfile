# Using cargo-chef to cache dependencies
FROM rust:1.78.0-slim-bookworm as chef
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.78.0-slim-bookworm as cacher
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler
RUN cargo install cargo-chef
COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build the application
FROM rust:1.78.0-slim-bookworm as builder
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/bourso /usr/local/bin/
EXPOSE 3000
CMD ["/usr/local/bin/bourso"]
