#!/usr/bin/env bash
# Use bash because because it has a built-in 'time' command.

. "$(dirname "$0")"/lib.sh

check_crate() {
  cd "$1/"
  if [ "$TOOLCHAIN" != '+nightly' ]; then
    cargo_fmt_clippy
    # Once cargo-geiger builds on nightly,
    # change this to always check the readme.
    # https://github.com/rust-secure-code/cargo-geiger/issues/181
    check_readme
  fi
  time $CARGO publish --dry-run "$ALLOW_DIRTY"
}

check_all() {
  cargo_check_build_test
  time check_crate safe-regex-compiler
  time check_crate safe-regex-macro
  time check_crate safe-regex
  echo "$0 finished"
}

set -e
cd "$TOP_LEVEL_DIR"
set -x
time check_all "$@"
