#!/usr/bin/env sh

set -e
set -o pipefail

cargo bench --features parser "$@"
