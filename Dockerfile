FROM rust:1.79.0 AS builder

WORKDIR /usr/src/app
COPY media_fetch_bot/src ./src
COPY media_fetch_bot/locales ./locales
COPY media_fetch_bot/.env ./
COPY media_fetch_bot/Cargo.lock ./
COPY media_fetch_bot/Cargo.toml ./

RUN cargo fetch
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt update && apt install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/media_fetch_bot /usr/local/bin/
COPY --from=builder /usr/src/app/.env /usr/local/bin/

CMD ["ls", "-l"]

WORKDIR /usr/local/bin
ENTRYPOINT ["media_fetch_bot"]

# Optionally, specify a command to run the bot
CMD ["media_fetch_bot"]