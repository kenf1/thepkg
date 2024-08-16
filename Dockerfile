FROM rust:1.81-slim

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    pkg-config \
    openssl \
    libssl-dev
RUN cargo install cargo-watch