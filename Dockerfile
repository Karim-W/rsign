FROM rust:1.65 AS builder
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder ./target/release/rsign ./target/release/rsign
EXPOSE 8000
CMD ["/target/release/rsign"]
