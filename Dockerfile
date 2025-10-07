# Build stage
FROM rust:1.72 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rust-simple-web-server /usr/local/bin/rust-simple-web-server
EXPOSE 7878
CMD ["/usr/local/bin/rust-simple-web-server"]
