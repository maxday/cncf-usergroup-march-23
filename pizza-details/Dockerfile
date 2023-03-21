FROM rust:1.68.0 as builder
RUN apt-get update
RUN apt-get install -y protobuf-compiler
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/pizza-details /usr/local/bin/pizza-details
CMD ["pizza-details"]