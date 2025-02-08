FROM rust:latest AS builder
WORKDIR /app
COPY . .

RUN cargo build --release


FROM debian:latest

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /app/target/release/fibbonacci /app/fibbonacci

CMD ["/app/fibbonacci"]
