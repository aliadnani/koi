FROM rust:1.64 AS builder
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder ./target/release/koi ./target/release/koi
ENTRYPOINT  ["/target/release/koi"]