#!/usr/bin/env sh
top_level_dir=$(
  cd "$(dirname "$0")"
  pwd
)
set -e
set -x
"$top_level_dir"/check-all.sh +stable "$@"
"$top_level_dir"/check-all.sh +nightly "$@"
