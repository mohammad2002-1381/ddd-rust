FROM rust:1.93 AS builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release -p api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

ENV RUST_LOG=debug

COPY --from=builder /app/target/release/api ./api

COPY --from=builder /app/.env ./.env

EXPOSE 5000
ENTRYPOINT ["./api"]
