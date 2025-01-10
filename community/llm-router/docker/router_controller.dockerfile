FROM rust:latest AS builder


RUN cargo install cargo-watch
COPY router_controller/crates/llm-router-gateway-api/ /router_controller/crates/llm-router-gateway-api/

WORKDIR /router_controller
