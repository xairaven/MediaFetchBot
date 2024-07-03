FROM rust:1.79.0 AS builder

WORKDIR /usr/src/app
COPY src ./src
COPY locales ./locales
COPY .env ./
COPY Cargo.lock ./
COPY Cargo.toml ./

RUN cargo fetch
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update && apt install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/media_fetch_bot /usr/local/bin/
COPY --from=builder /usr/src/app/.env /usr/local/bin/

WORKDIR /usr/local/bin
ENTRYPOINT ["media_fetch_bot"]

# Optionally, specify a command to run the bot
CMD ["media_fetch_bot"]