FROM rust:1.37.0-slim-buster

# Install additional packages
RUN apt-get update \
    && apt-get install -y \
        libssl-dev \
        pkg-config \
        curl \
    && apt-get autoremove \
    && rm -rf /var/lib/apt/lists/*

# TODO checksum
RUN curl -sSL https://github.com/mozilla/sccache/releases/download/0.2.12/sccache-0.2.12-x86_64-unknown-linux-musl.tar.gz | tar -zx \
    && mv sccache-0.2.12-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && rm -rf sccache-0.2.12-x86_64-unknown-linux-musl

# Use correct toolchain, install WASM components required by the
# ledger, and Rust utilities necessary in some CI pipeline steps i.e.
# formatting check, linting.
RUN rustup default nightly-2019-09-05 \
    && rustup update \
    && rustup component add rustfmt \
    && rustup component add clippy \
    && rustup target add wasm32-unknown-unknown

# Install Parity Ethereum node.
RUN curl -sSLO https://releases.parity.io/ethereum/v2.6.0/x86_64-unknown-linux-gnu/parity \
    && echo "1b50cabc8ce54983d1b10be4c4f5887ff4ecfe63177e6c49cde819a563fb9d96  parity" > parity.sum \
    && sha256sum parity.sum \
    && chmod +x parity \
    && mv parity /usr/local/bin

# Setup defaults for caching
VOLUME /cache
ENV SCCACHE_DIR=/cache/sccache RUSTC_WRAPPER=sccache CARGO_HOME=/cache/cargo
