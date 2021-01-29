#!/usr/bin/env bash
# This script uses bash because it has a built-in 'time' command.
# The rust docker images have no 'time' binary and use dash for 'sh' which has
# no built-in 'time' command.

check_cargo_readme() {
  cargo readme >Readme.md.tmp
  ls -alF $CI_PROJECT_DIR
  cargo geiger --update-readme --readme-path Readme.md.tmp --output-format GitHubMarkdown
  diff Readme.md Readme.md.tmp || (
    echo "Readme.md is stale" >&2
    exit 1
  )
  rm -f Readme.md.tmp
  git rm -f --ignore-unmatch Readme.md.tmp
}

check() {
  ls -alF $CI_PROJECT_DIR
  time cargo check --verbose
  ls -alF $CI_PROJECT_DIR
  time cargo build --verbose
  ls -alF $CI_PROJECT_DIR
  time cargo test --verbose
  ls -alF $CI_PROJECT_DIR
  time cargo fmt --all -- --check
  ls -alF $CI_PROJECT_DIR
  time cargo clippy --all-targets --all-features -- -D clippy::pedantic
  ls -alF $CI_PROJECT_DIR
  #time check_cargo_readme
  ls -alF $CI_PROJECT_DIR
  time cargo publish --dry-run "$@"
  echo "$0 finished"
  ls -alF $CI_PROJECT_DIR
}
set -e
set -x
time check "$@"
