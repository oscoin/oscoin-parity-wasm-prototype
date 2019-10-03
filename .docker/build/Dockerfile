FROM rust:1.37.0-slim-buster

# Install additional packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends wget \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

RUN cargo build --package pwasm-utils-cli --bin wasm-build \
    && rm -rf /usr/local/cargo/registry \
    && rm /usr/local/cargo/.package-cache

# Install Parity Ethereum.
RUN wget https://releases.parity.io/ethereum/v2.6.0/x86_64-unknown-linux-gnu/parity \
    && echo "1b50cabc8ce54983d1b10be4c4f5887ff4ecfe63177e6c49cde819a563fb9d96  parity" > parity.sum \
    && sha256sum parity.sum \
    && chmod +x parity \
    && sudo mv parity /usr/local/bin

# Setup defaults for caching
VOLUME /cache
ENV SCCACHE_DIR=/cache/sccache RUSTC_WRAPPER=sccache CARGO_HOME=/cache/cargo