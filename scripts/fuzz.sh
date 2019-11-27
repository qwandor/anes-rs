#!/usr/bin/env sh

set -e
set -o pipefail

OUT_DIR="fuzzer/out"

if [ ! -d "$OUT_DIR" ]; then
  mkdir -p "$OUT_DIR"
fi

cargo afl build
cargo afl fuzz -i fuzzer/in -o fuzzer/out target/debug/fuzz-parser-advance "$@"
