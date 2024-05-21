FROM rust:latest AS builder
WORKDIR /usr/src/flights-app
COPY . .
WORKDIR /usr/src/flights-app/flights-monitor
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM ubuntu:jammy
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
WORKDIR /usr/src/flights-monitor
COPY --from=builder /usr/src/flights-app/target/release/flights-monitor .
COPY --from=builder /usr/src/flights-app/flights-monitor/config.yaml .
ENTRYPOINT ["./flights-monitor"]

