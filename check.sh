#!/usr/bin/env bash
# This script uses bash because it has a built-in 'time' command.
# The rust docker images have no 'time' binary and use dash for 'sh' which has
# no built-in 'time' command.

check_cargo_readme() {
  cargo readme >Readme.md.tmp
  diff Readme.md Readme.md.tmp || (
    echo "Readme.md is stale" >&2
    exit 1
  )
  rm -f Readme.md.tmp
  git rm -f --ignore-unmatch Readme.md.tmp
}

check() {
  time cargo check --verbose
  time cargo build --verbose
  time cargo test --verbose
  time cargo fmt --all -- --check
  time cargo clippy --all-targets --all-features -- -D clippy::pedantic
  time check_cargo_readme
  time cargo publish --dry-run "$@"
  echo "$0 finished"
}
set -e
set -x
time check "$@"
