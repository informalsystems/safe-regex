TOP_LEVEL_DIR=$(
  cd "$(dirname "$0")"
  pwd
)

usage() {
  echo "$(basename "$0")": ERROR: "$@" 1>&2
  echo usage: "$(basename "$0")" '[+nightly|+stable] [--allow-dirty]' 1>&2
  exit 1
}

TOOLCHAIN=
ALLOW_DIRTY=

while :; do
  case "$1" in
  +*) TOOLCHAIN="$1" ;;
  --allow-dirty) ALLOW_DIRTY=--allow-dirty ;;
  '') break ;;
  *) usage "bad argument '$1'" ;;
  esac
  shift
done

CARGO="cargo $TOOLCHAIN"

generate_readme() {
  set -e
  set -x
  $CARGO readme >"$1"
  $CARGO geiger --update-readme --readme-path "$1" --output-format GitHubMarkdown
}

check_readme() {
  set -e
  generate_readme Readme.md.tmp
  set -x
  diff Readme.md Readme.md.tmp || (
    echo "Readme.md is stale" >&2
    exit 1
  )
  rm -f Readme.md.tmp
  git rm -f --ignore-unmatch Readme.md.tmp
}

cargo_check_build_test() {
  set -e
  set -x
  time $CARGO check --verbose
  time $CARGO build --verbose
  time $CARGO test --verbose
}

cargo_fmt_clippy() {
  set -e
  set -x
  time $CARGO fmt -- --check
  time $CARGO clippy --all-features -- -D clippy::pedantic
}

cargo_publish_dryrun() {
  set -e
  set -x
  time $CARGO publish --dry-run $ALLOW_DIRTY
}
