#!/usr/bin/env bash

set -euo pipefail

docker run \
  --read-only \
  --rm \
  --workdir /project \
  --volume $(pwd):/project \
  --volume /tmp \
  gcr.io/opensourcecoin/oscoin-pwasm-ci-base \
  cargo build
