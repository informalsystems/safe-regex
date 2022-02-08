#!/usr/bin/env bash
# Use bash because because it has a built-in 'time' command.
set -e
cd "$(dirname "$0")"
echo "PWD=$(pwd)"
time (
  set -e
  set -x
  ./check.sh safe-regex-compiler "$@"
  ./check.sh safe-regex-macro "$@"
  ./check.sh safe-regex "$@"
  # TODO: Build benchmark.
  # TODO: Run benchmark and check for regressions.
  set +x
  echo -n "$(basename "$0") finished."
)
