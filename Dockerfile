FROM rust:1.87 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM ubuntu:22.04
RUN apt-get update -y && \
    apt-get install -y libssl-dev ca-certificates && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/prompt-shelf /app
RUN chmod +x /app

ENTRYPOINT ["/app"]
