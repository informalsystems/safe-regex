#!/usr/bin/env bash
set -e
cd "$(dirname $0)"
du_before=$(du -sh | cut -f 1)
(
  set -x
  cargo clean
  rm -f Cargo.lock
)
du_after=$(du -sh | cut -f 1)
echo "$du_before -> $du_after"
