FROM rust:1.71 as builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev musl-tools

WORKDIR /app
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/discord_bot /usr/local/bin/discord_bot

CMD ["discord_bot"]
