FROM rust:latest AS builder
WORKDIR /usr/src/flights-app
COPY . .
WORKDIR /usr/src/flights-app/flights-data
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /usr/src/flights-data
COPY --from=builder /usr/src/flights-app/target/release/flights-data .
ENTRYPOINT ["./flights-data"]

