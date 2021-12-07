#!/bin/bash

set -o errexit
set -o pipefail

fail() {
    echo "$@" >&2
    exit 1
}

cd $(git rev-parse --show-toplevel)
echo "Checking that everything builds"
cargo check --frozen || fail "COMPILE CHECK FAILED"

echo "Checking that tests pass"
cargo test || fail "ONE OR MORE TESTS FAILED"

echo "Checking code formatting"
cargo fmt -- --check || fail "CODE NEEDS TO BE FORMATTED"
