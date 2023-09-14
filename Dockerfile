FROM rust as builder
WORKDIR /app
COPY Cargo.* ./
COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
RUN apt-get install -y openssl
COPY --from=builder /app/target/release/playstore-crawler /usr/local/bin/playstore-crawler
CMD ["playstore-crawler"]
