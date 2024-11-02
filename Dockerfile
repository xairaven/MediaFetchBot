FROM rust:1.82.0 AS builder

# Copying code + .env
WORKDIR /usr/src/app
COPY src ./src
COPY locales ./locales
COPY .env ./
COPY whitelist.json* ./
COPY Cargo.toml ./

# BUILD
RUN cargo fetch
RUN cargo build --release

# Runner image
FROM debian:bookworm-slim

# Essential packages
RUN apt update && apt install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

########################################################
# ENV VARIABLES

# Your app name (write same name to both)
ARG APP_NAME_ARG=media_fetch_bot
ENV APP_NAME_ENV=media_fetch_bot

########################################################

# Copying built executable from builder to runner
COPY --from=builder /usr/src/app/target/release/${APP_NAME_ARG} /usr/local/bin/
COPY --from=builder /usr/src/app/.env /usr/local/bin/
COPY --from=builder /usr/src/app/whitelist.json* /usr/local/bin/

# Workdir on runner
WORKDIR /usr/local/bin

# Use the exec form of ENTRYPOINT with a shell to expand environment variables
ENTRYPOINT "./$APP_NAME_ENV"