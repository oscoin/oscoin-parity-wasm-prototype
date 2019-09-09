# Base image for building the project on CI.
#
# Based on debian:buster-slim and includes
# * Git
# * Rustup with default toolchain from RUST_TOOLCHAIN build arg
# * Rustup components and targets from ../tools/rustup-setup
# * Parity 2.6.0
FROM debian:buster-slim

# Rust toolchain to install with rustup. The build script obtains this from
# `../rust-toolchain`.
ARG RUST_TOOLCHAIN

WORKDIR /project

ENV RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        git \
        libc6-dev \
        libssl-dev \
        pkg-config \
        wget \
        ; \
    \
    wget https://static.rust-lang.org/rustup/archive/1.19.0/x86_64-unknown-linux-gnu/rustup-init; \
    echo "36285482ae5c255f2decfab27d32ba19465804cb3ddf5a23e6ff2a7b0f6e0250  rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    CARGO_HOME=/usr/local/cargo ./rustup-init -y --no-modify-path --default-toolchain none; \
    rm rustup-init; \
    \
    wget https://releases.parity.io/ethereum/v2.6.0/x86_64-unknown-linux-gnu/parity; \
    echo "1b50cabc8ce54983d1b10be4c4f5887ff4ecfe63177e6c49cde819a563fb9d96  parity" | sha256sum; \
    chmod +x parity; \
    mv parity /usr/local/bin; \
    \
    apt-get autoremove -y wget; \
    rm -rf /var/lib/apt/lists/*; \
    chmod -R a+w $RUSTUP_HOME;

ADD rustup-setup .
RUN \
  rustup install --no-self-update $RUST_TOOLCHAIN && \
  rustup default $RUST_TOOLCHAIN && \
  ./rustup-setup && \
  rm ./rustup-setup
