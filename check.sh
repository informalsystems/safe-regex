#!/usr/bin/env bash
# Use bash because because it has a built-in 'time' command.
set -e

script_dir="$(
  cd "$(dirname "$0")"
  pwd
)"

usage() {
  echo "$(basename "$0")": ERROR: "$@" >&2
  echo usage: "$(basename "$0")" '[DIRECTORY] [--skip-readme-check]' >&2
  exit 1
}

while :; do
  case "$1" in
  --skip-readme-check) skip_readme_check=1 ;;
  +nightly) CARGO_CHANNEL=+nightly ;;
  '') break ;;
  -*) usage "bad argument '$1'" ;;
  *)
    [ -z "$DIR" ] || usage "unexpected argument: '$1'"
    DIR="$1"
    cd "$script_dir/$1"
    ;;
  esac
  shift
done

cargo="cargo $CARGO_CHANNEL"

echo "Running cargo check."
echo "PWD=$(pwd)"
time (
  set -x
  $cargo check --verbose
  set +x
  echo -n "Cargo check done."
)

echo ''
# TODO(https://github.com/rust-secure-code/cargo-geiger/issues/181)
#     Run cargo-geiger on nightly once it builds on nightly.
if rustc --version | grep --quiet -- '-nightly'; then
  echo "Nightly detected.  Skipping clippy and readme checks."
else
  echo "Running cargo fmt check."
  echo "PWD=$(pwd)"
  time (
    set -x
    $cargo fmt -- --check
    set +x
    echo -n "Finished cargo fmt check."
  )

  echo ''
  echo "Running cargo clippy."
  echo "PWD=$(pwd)"
  package=$($cargo read-manifest | tr ',' '\n' | head -n 1 | cut -d: -f2 | cut -d '"' -f 2)
  set -x
  $cargo clean --package "$package"
  $cargo clippy --all-features -- -D clippy::pedantic --no-deps
  set +x

  if [ -n "$skip_readme_check" ]; then
    echo "Skipping readme check."
  else
    echo ''
    "$script_dir"/generate-readme.sh --filename Readme.md.tmp
    echo ''
    echo "Checking if source readme matches generated one."
    echo "PWD=$(pwd)"
    set -x
    diff Readme.md Readme.md.tmp >&2 || exit 1
    rm -f Readme.md.tmp
    git rm -f --ignore-unmatch Readme.md.tmp
    set +x
  fi
fi

echo ''
echo "Running cargo build."
echo "PWD=$(pwd)"
time (
  set -x
  $cargo build --verbose
  set +x
  echo -n "Finished cargo build."
)

echo ''
echo "Running cargo test."
echo "PWD=$(pwd)"
time (
  set -x
  $cargo test --verbose
  set +x
  echo -n "Finished cargo test."
)

echo ''
if grep --quiet 'publish = false' Cargo.toml; then
  echo "Found publish=false in Cargo.toml.  Skipping cargo publish dry run."
else
  echo "Running cargo publish dry run."
  echo "PWD=$(pwd)"
  time (
    set -x
    $cargo publish --dry-run --allow-dirty
    set +x
    echo -n "Finished cargo publish dry run."
  )
fi

echo
