#!/usr/bin/env bash
# Use bash because because it has a built-in 'time' command.

. "$(dirname "$0")"/lib.sh

check_crate() {
  cd "$TOP_LEVEL_DIR/$1"
  if [ "$TOOLCHAIN_ARG" != '+nightly' ]; then
    cargo_fmt_clippy
    # Once cargo-geiger builds on nightly,
    # change this to always check the readme.
    # https://github.com/rust-secure-code/cargo-geiger/issues/181
    check_readme
  fi
  cargo_publish_dryrun
}

check_all() {
  cd "$TOP_LEVEL_DIR"
  cargo_check_build_test
  time check_crate safe-regex-compiler
  time check_crate safe-regex-macro
  time check_crate safe-regex
  echo "$0 finished"
}

set -e
set -x
time check_all "$@"
