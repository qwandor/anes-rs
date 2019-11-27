#!/usr/bin/env sh

set -e
set -o pipefail

OUT_DIR="fuzzer/out"

if [ ! -d "$OUT_DIR" ]; then
  echo "No output dir, you can't continue with fuzzing"
  exit -1
fi

cargo afl fuzz -i - -o "$OUT_DIR" target/debug/fuzz-parser-advance "$@"
