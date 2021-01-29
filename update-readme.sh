#!/usr/bin/env sh
if [ -f Readme.md ]; then
  set -e
  cargo +nightly readme >Readme.md
else
  echo "Readme.md does not exist. Check your directory." >&2
  exit 1
fi
