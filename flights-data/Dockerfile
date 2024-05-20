FROM rust:latest AS builder
WORKDIR /usr/src/flights-app
COPY . .
WORKDIR /usr/src/flights-app/flights-data
RUN cargo build --release

FROM ubuntu:jammy
WORKDIR /usr/src/flights-data
COPY --from=builder /usr/src/flights-app/target/release/flights-data .
COPY --from=builder /usr/src/flights-app/flights-data/config.yaml .
ENTRYPOINT ["./flights-data"]
CMD ["migrate"]

