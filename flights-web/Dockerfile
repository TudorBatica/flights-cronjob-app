# Build app
FROM rust:latest AS builder
WORKDIR /usr/src/flights-app
COPY . .
WORKDIR /usr/src/flights-app/flights-web
ENV SQLX_OFFLINE true
RUN cargo build --release

# Install 
FROM ubuntu:jammy
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
WORKDIR /usr/src/flights-web
COPY --from=builder /usr/src/flights-app/target/release/flights-web .
COPY --from=builder /usr/src/flights-app/flights-web/config.yaml .

# Run configuration
EXPOSE 8000
ENTRYPOINT ["./flights-web"]

