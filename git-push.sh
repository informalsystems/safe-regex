#!/usr/bin/env sh
top_level_dir=$(
  cd "$(dirname "$0")"
  pwd
)
set -e
set -x
(
  cd "$top_level_dir"/
  "$top_level_dir"/check-all.sh "$@"
  git push --follow-tags
)
