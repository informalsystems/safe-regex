#!/usr/bin/env sh
set -e
set -x
"$(dirname "$0")"/check-all.sh +stable "$@"
"$(dirname "$0")"/check-all.sh +nightly "$@"
git push --follow-tags
