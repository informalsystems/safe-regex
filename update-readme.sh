#!/usr/bin/env sh
if [ -f Readme.md ]; then
  set -e
  set -x
  cargo +nightly readme >Readme.md
  cargo geiger --update-readme --readme-path Readme.md --output-format GitHubMarkdown
else
  echo "Readme.md does not exist. Check your directory." >&2
  exit 1
fi
