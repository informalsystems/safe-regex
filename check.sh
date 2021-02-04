#!/usr/bin/env bash
# This script uses bash because it has a built-in 'time' command.
# The rust docker images have no 'time' binary and use dash for 'sh' which has
# no built-in 'time' command.

usage() {
  echo "$(basename "$0")": ERROR: "$@" 1>&2
  echo usage: "$(basename "$0")" '[+nightly|+stable] [--allow-dirty]' 1>&2
  exit 1
}

toolchain=+stable
allow_dirty=

while :; do
  case "$1" in
  +nightly) toolchain=+nightly ;;
  +stable) toolchain=+stable ;;
  --allow-dirty) allow_dirty=--allow-dirty ;;
  '') break;;
  *) usage "bad argument '$1'" ;;
  esac
  shift
done

CARGO="cargo $toolchain"

check_cargo_readme() {
  if [ "$toolchain" == '+nightly' ]; then
    echo "Skipping checking readme because of '$toolchain' argument."
    return 0
  fi
  $CARGO readme >Readme.md.tmp
  # Once cargo-geiger-serde builds on nightly,
  # change this to always run `cargo geiger`.
  # https://github.com/rust-secure-code/cargo-geiger/issues/181
  $CARGO geiger --update-readme --readme-path Readme.md.tmp --output-format GitHubMarkdown
  diff Readme.md Readme.md.tmp || (
    echo "Readme.md is stale" >&2
    exit 1
  )
  rm -f Readme.md.tmp
  git rm -f --ignore-unmatch Readme.md.tmp
}

check() {
  time $CARGO check --verbose
  time $CARGO build --verbose
  time $CARGO test --verbose
  time $CARGO fmt --all -- --check
  time $CARGO clippy --all-targets --all-features -- -D clippy::pedantic
  time check_cargo_readme
  time $CARGO publish --dry-run $allow_dirty
  echo "$0 finished"
}

set -e
set -x
time check "$@"
