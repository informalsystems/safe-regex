TOP_LEVEL_DIR=$(
  cd "$(dirname "$0")"
  pwd
)

usage() {
  echo "$(basename "$0")": ERROR: "$@" 1>&2
  echo usage: "$(basename "$0")" '[+nightly|+stable] [--allow-dirty]' 1>&2
  exit 1
}

TOOLCHAIN_ARG=
ALLOW_DIRTY_ARG=

while :; do
  case "$1" in
  +*) TOOLCHAIN_ARG="$1" ;;
  --allow-dirty) ALLOW_DIRTY_ARG=--allow-dirty ;;
  '') break ;;
  *) usage "bad argument '$1'" ;;
  esac
  shift
done

CARGO_CMD="cargo $TOOLCHAIN_ARG"

generate_readme() {
  set -e
  set -x
  time $CARGO_CMD readme >"$1"
  time $CARGO_CMD geiger --update-readme --readme-path "$1" --output-format GitHubMarkdown
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
  time $CARGO_CMD check --verbose
  time $CARGO_CMD build --verbose
  time $CARGO_CMD test --verbose
}

cargo_fmt_clippy() {
  set -e
  set -x
  time $CARGO_CMD fmt -- --check
  PACKAGE_NAME=$(basename "$PWD")
  time $CARGO_CMD clean --package "$PACKAGE_NAME"
  time $CARGO_CMD clippy --all-features -- -D clippy::pedantic --no-deps
}

cargo_publish_dryrun() {
  set -e
  set -x
  time $CARGO_CMD publish --dry-run $ALLOW_DIRTY_ARG
}
