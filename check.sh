#!/usr/bin/env bash
# Use bash because because it has a built-in 'time' command.

. "$(dirname "$0")"/lib.sh

check() {
  set -e
  set -x
  cargo_check_build_test
  if [ "$TOOLCHAIN" != '+nightly' ]; then
    cargo_fmt_clippy
    # Once cargo-geiger builds on nightly,
    # change this to always check the readme.
    # https://github.com/rust-secure-code/cargo-geiger/issues/181
    check_readme
  fi
  cargo_publish_dryrun
  echo "$0 finished"
}

set -e
set -x
time check "$@"
