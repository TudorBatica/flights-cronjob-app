FROM rust:latest AS builder
WORKDIR /usr/src/flights-app
COPY . .
WORKDIR /usr/src/flights-app/flights-monitor
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM ubuntu:jammy
WORKDIR /usr/src/flights-monitor
COPY --from=builder /usr/src/flights-app/target/release/flights-monitor .
COPY --from=builder /usr/src/flights-app/flights-monitor/config.yaml .
ENTRYPOINT ["./flights-monitor"]

