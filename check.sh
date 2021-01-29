#!/usr/bin/env bash
# This script uses bash because it has a built-in 'time' command.
# The rust docker images have no 'time' binary and use dash for 'sh' which has
# no built-in 'time' command.

check_cargo_readme() {
  cargo readme >Readme.md.tmp
  ls -alF "$CI_PROJECT_DIR" || true
  # Once cargo-geiger-serde builds on nightly,
  # change this to always run `cargo geiger`.
  # https://github.com/rust-secure-code/cargo-geiger/issues/181
  if [ "$CI_JOB_NAME" != "rust-nightly" ]; then
    cargo geiger --update-readme --readme-path Readme.md.tmp --output-format GitHubMarkdown
  fi
  diff Readme.md Readme.md.tmp || (
    echo "Readme.md is stale" >&2
    exit 1
  )
  rm -f Readme.md.tmp
  git rm -f --ignore-unmatch Readme.md.tmp
}

check() {
  ls -alF "$CI_PROJECT_DIR" || true
  time cargo check --verbose
  ls -alF "$CI_PROJECT_DIR" || true
  time cargo build --verbose
  ls -alF "$CI_PROJECT_DIR" || true
  time cargo test --verbose
  ls -alF "$CI_PROJECT_DIR" || true
  time cargo fmt --all -- --check
  ls -alF "$CI_PROJECT_DIR" || true
  time cargo clippy --all-targets --all-features -- -D clippy::pedantic
  ls -alF "$CI_PROJECT_DIR" || true
  #time check_cargo_readme
  ls -alF "$CI_PROJECT_DIR" || true
  time cargo publish --dry-run "$@"
  echo "$0 finished"
  ls -alF "$CI_PROJECT_DIR" || true
}
set -e
set -x
time check "$@"
