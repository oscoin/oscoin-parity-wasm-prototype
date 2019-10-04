FROM rust:1.38.0-slim

# Install additional packages
FROM ubuntu:18.04
RUN apt-get update \
    && apt-get install -y wget curl \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

# Install Parity Ethereum.
RUN wget https://releases.parity.io/ethereum/v2.6.0/x86_64-unknown-linux-gnu/parity \
    && echo "1b50cabc8ce54983d1b10be4c4f5887ff4ecfe63177e6c49cde819a563fb9d96  parity" > parity.sum \
    && sha256sum parity.sum \
    && chmod +x parity \
    && mv parity /usr/local/bin

# Setup defaults for caching
VOLUME /cache
ENV SCCACHE_DIR=/cache/sccache RUSTC_WRAPPER=sccache CARGO_HOME=/cache/cargo