FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM nvcr.io/nvidia/base/ubuntu:22.04_20240212
COPY --from=builder /app/target/release/llm-router-classification /usr/local/bin/
RUN mkdir -p /app
WORKDIR /app

ENTRYPOINT ["llm-router-classification"]
CMD [""]
